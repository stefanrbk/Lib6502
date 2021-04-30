use super::*;

use predecoder::Predecoder;
use std::{thread, thread::JoinHandle};

mod predecoder;

pub struct Cpu {
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    _p: StatusFlags,
    dor: u8,
    dl: u8,
    pcls: u8,
    pcl: u8,
    abl: u8,
    abh: u8,
    pchs: u8,
    pch: u8,
    db: u8,
    adl: u8,
    adh: u8,
    sb: u8,
    irq_rst_control: IrqRstControl,
    ready_control: ReadyControl,
    predecoder: Predecoder,
    decoder: Decoder,
    timing_control: TimingControl,
    alu: Alu,
    io: CpuIO,
}

impl Cpu {
    pub fn get_p(self) -> u8 {
        self._p.0
    }

    pub fn set_p(&mut self, value: u8) {
        self._p.0 = value & 0b11011111;
    }

    #[export_name = "new_cpu"]
    pub fn new(io: CpuIO) -> Cpu {
        Cpu {
            s: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            _p: StatusFlags { 0: 0 },
            dor: 0,
            dl: 0,
            pcls: 0,
            pcl: 0,
            abl: 0,
            abh: 0,
            pchs: 0,
            pch: 0,
            db: 0,
            adl: 0,
            adh: 0,
            sb: 0,
            irq_rst_control: IrqRstControl::new(),
            ready_control: ReadyControl::new(),
            predecoder: Predecoder::new(),
            decoder: Decoder { 0: 0 },
            timing_control: TimingControl::new(),
            alu: Alu::new(),
            io: io,
        }
    }

    #[export_name = "start_cpu"]
    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            set_pin!(self.io.rw);

            loop {
                self.cycle();
            }
        })
    }

    pub fn trigger_nmi(&mut self) {
        clear_pin!(self.io.nmi);
    }

    pub fn clear_nmi(&mut self) {
        set_pin!(self.io.nmi);
    }

    pub fn trigger_rst(&mut self) {
        clear_pin!(self.io.rst);
    }

    pub fn clear_rst(&mut self) {
        set_pin!(self.io.rst);
    }

    pub fn trigger_irq(&mut self) {}

    pub fn clear_irq(&mut self) {}

    fn cycle(&mut self) {
        self.io.phase_1_positive_edge.wait();

        self.phase_1();

        self.io.phase_1_negative_edge.wait();

        self.io.phase_2_positive_edge.wait();

        self.phase_2();

        self.io.phase_2_negative_edge.wait();
    }

    fn phase_1(&mut self) {
        self.ready_control.phase_1(&self.io);
        self.irq_rst_control.phase_1();
        self.timing_control.phase_1(
            &mut self.io,
            &self.alu,
            &self.decoder,
            &self.irq_rst_control,
            &self.ready_control,
        );
        self.predecoder.phase_1(&self.timing_control);
        self.decoder = Decoder::new(self.predecoder.get_ir(), &self.timing_control);
    }
    fn phase_2(&mut self) {
        // Update input data latch
        self.dl = read_bus!(self.io.db);

        self.ready_control.phase_2(&self.io);
        self.irq_rst_control.phase_2(&mut self._p, &self.io);
        self.timing_control
            .phase_2(&self.irq_rst_control, &self.ready_control);
        // Update predecoder
        self.predecoder.set_pd(self.dl);

        self.predecoder
            .phase_2(self.db, &self.timing_control, &self.irq_rst_control);
    }
}

impl super::Alu {
    pub fn new() -> Alu {
        Alu { 0: 0 }
    }
}

impl super::ReadyControl {
    pub fn new() -> ReadyControl {
        ReadyControl { 0: 0 }
    }
    fn phase_1(&mut self, io: &CpuIO) {
        let rdy = read_pin!(io.rdy);
        self.set_not_rdy(!rdy);

        self.set_hold_branch(self.get_not_rdy_last_phase_2());
        self.set_rdy_last_phase_1(rdy);
    }
    fn phase_2(&mut self, io: &CpuIO) {
        let rdy = read_pin!(io.rdy);
        self.set_not_rdy(!rdy);

        self.set_not_rdy_last_phase_2(!rdy);
        self.set_not_rdy_delay(!self.get_rdy_last_phase_1());
    }
}

impl super::TimingControl {
    pub fn new() -> TimingControl {
        TimingControl { 0: 0 }
    }
    pub fn get_tstate(&self) -> u8 {
        0
    }
    fn phase_1(
        &mut self,
        io: &mut CpuIO,
        alu: &Alu,
        decoder: &Decoder,
        rst: &IrqRstControl,
        rc: &ReadyControl,
    ) {
        // c
        {
            let c = !(self.short_circuit_branch_add(alu, decoder, rc) || self.get_unk_20());
            self.set_c(c);
        }
        // a
        {
            self.set_a(!(rc.get_not_rdy() || self.get_c()));
        }

        // Sync
        {
            if self.not_sync(alu, decoder, rc) {
                clear_pin!(io.sync);
            } else {
                set_pin!(io.sync);
            }
        }
    }
    fn phase_2(&mut self, rst: &IrqRstControl, rc: &ReadyControl) {
        // Fetch
        {
            self.set_fetch(!rc.get_not_rdy() && self.get_sync());
        }
        self.set_sync_last_phase_2(self.get_sync());
    }
    fn not_sync(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !self.sync_left(alu, decoder, rc) && !self.get_b()
    }
    fn sync_left(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !rc.get_not_rdy() && !self.not_rdy_lower(alu, decoder, rc)
    }
    fn not_rdy_lower(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !self.short_circuit_branch_add(alu, decoder, rc) && !self.get_unk_20()
    }
    fn short_circuit_branch_add(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        self.not_t3_branch_or_not_rdy_delay(decoder, rc)
            && (self.get_branch_back_phase_1() != alu.get_alu_c_out())
    }
    fn not_t3_branch_or_not_rdy_delay(&self, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !(decoder.get_t3_branch() || rc.get_not_rdy_delay())
    }
}

impl super::Decoder {
    pub fn new(ir: u8, tstate: &TimingControl) -> Decoder {
        let mut value = 0 as u128;

        for i in (0..=127).rev() {
            if pla::check_opcode(ir, tstate.get_tstate(), pla::PLA[i]) {
                value |= 1;
            }
            value <<= 1;
        }

        Decoder { 0: value }
    }
}

impl super::CpuIO {
    pub fn new() -> CpuIO {
        CpuIO {
            db: new_bus!(0 as u8),
            abh: new_bus!(0 as u8),
            abl: new_bus!(0 as u8),
            rw: new_pin_set!(),
            irq: new_pin_set!(),
            rdy: new_pin_unset!(),
            nmi: new_pin_set!(),
            rst: new_pin_unset!(),
            sync: new_pin_unset!(),
            so: new_pin_unset!(),
            phase_1_negative_edge: null_barrier!(),
            phase_1_positive_edge: null_barrier!(),
            phase_2_negative_edge: null_barrier!(),
            phase_2_positive_edge: null_barrier!(),
            read_write_negative_edge: null_barrier!(),
            read_write_positive_edge: null_barrier!(),
            sync_negative_edge: null_barrier!(),
            sync_positive_edge: null_barrier!(),
            addr_stable: null_barrier!(),
            data_stable: null_barrier!(),
        }
    }
}

mod pla;
