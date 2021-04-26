use super::*;
use std::{thread, thread::JoinHandle};

impl super::Cpu {
    pub fn get_p(self) -> u8 {
        self._p.0
    }

    pub fn set_p(&mut self, value: u8) {
        self._p.0 = value & 0b11011111;
    }

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
            ai: 0,
            bi: 0,
            add: 0,
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
            decoder: Decoder::new(),
            timing_control: TimingControl::new(),
            t_state: TState::Kil,
            io: io,
        }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.t_state = TState::T0;
            set_pin!(self.io.rw);

            loop {
                match self.t_state {
                    TState::Kil => break,
                    _ => self.cycle(),
                }
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
        self.ready_control.phase_1(&self.io);
        self.irq_rst_control.phase_1();
        self.phase_1();
        self.io.phase_1_negative_edge.wait();
        self.io.phase_2_positive_edge.wait();
        self.ready_control.phase_2(&self.io);
        self.irq_rst_control.phase_2(&mut self._p, &self.io);
        self.phase_2();
        self.io.phase_2_negative_edge.wait();
    }

    fn phase_1(&mut self) {
        self.predecoder
            .phase_1(&mut self.decoder, &self.timing_control);
    }
    fn phase_2(&mut self) {
        // Update input data latch
        self.dl = read_bus!(self.io.db);
        // Update predecoder
        self.predecoder.set_pd(self.dl);

        self.predecoder
            .phase_2(self.db, &self.timing_control, &self.irq_rst_control);
    }
}

impl super::Decoder {
    pub fn new() -> Decoder {
        Decoder { 0: 0 }
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
    }
    fn phase_2(&mut self, io: &CpuIO) {
        let rdy = read_pin!(io.rdy);

        self.set_not_rdy(!rdy);
        self.set_not_rdy_last_phase_2(!rdy);
    }
}

impl super::TimingControl {
    pub fn new() -> TimingControl {
        TimingControl { 0: 0 }
    }
    fn phase_1(&mut self, io: &CpuIO, rc: &ReadyControl) {
        if rc.get_not_rdy() {
            self.set_fetch(false);
        } else {
            self.set_fetch(self.get_do_fetch_last_phase_2());
        }
    }
    fn phase_2(&mut self, io: &CpuIO, rc: &ReadyControl) {
        self.set_do_fetch_last_phase_2(self.get_do_fetch());
        self.set_fetch(!rc.get_not_rdy() && self.get_do_fetch());
    }
}

impl super::Predecoder {
    pub fn new() -> Predecoder {
        Predecoder { 0: 0 }
    }

    fn phase_1(&mut self, decode: &mut Decoder, timing: &TimingControl) {
        if timing.get_fetch() {
            decode.set_ir(self.get_pd());
        }
    }

    fn phase_2(&mut self, db: u8, timing: &TimingControl, irq: &IrqRstControl) {
        self.set_pd(db);
        if timing.get_fetch() && !irq.irq_asserting() {
            self.clear_ir();
        } else {
            let pd_0xx0xx0x = pla::check_opcode(self.get_pd(), 0, false, pla::PD0XX0XX0X);
            let pd_1xx000x0 = pla::check_opcode(self.get_pd(), 0, false, pla::PD1XX000X0);
            let pd_xxx010x1 = pla::check_opcode(self.get_pd(), 0, false, pla::PDXXX010X1);
            let pd_xxxx10x0 = pla::check_opcode(self.get_pd(), 0, false, pla::PDXXXX10X0);

            self.set_two_cycle(pd_xxx010x1 || pd_1xx000x0 || (pd_xxxx10x0 && !pd_0xx0xx0x));
            self.set_one_byte(pd_xxxx10x0);
        }
    }
    pub fn clear_ir(&mut self) {
        self.set_two_cycle(false);
        self.set_one_byte(false);
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

impl super::IrqRstControl {
    pub fn new() -> IrqRstControl {
        IrqRstControl { 0: 0 }
    }

    pub fn phase_1(&mut self) {
        self.rst_phase_1();
        self.nmi_phase_1();
        self.irq_phase_1();
    }

    pub fn phase_2(&mut self, p: &mut StatusFlags, io: &CpuIO) {
        self.rst_phase_2(&io);
        self.nmi_phase_2(&io);
        self.irq_phase_2(&io);
        self.intg_phase_2(p);

        p.set_b(true); // Set b flag
    }

    fn intg_phase_2(&mut self, p: &StatusFlags) {
        // Set INTG
        {
            if (self.get_irqp() && p.get_i()) || self.get_nmip() {
                self.set_intg(true);
            }
        }
        // Unset INTG
        {
            if self.get_intg() && self.get_brk_done() {
                self.set_intg(false);
            }
        }
    }

    fn irq_phase_1(&mut self) {
        // Set IRQP
        {
            self.set_irqp(self.get_last_irq());
        }
    }

    fn irq_phase_2(&mut self, io: &CpuIO) {
        // Set IRQP
        {
            self.set_last_irq(!read_pin!(io.irq));
        }
    }

    fn nmi_phase_1(&mut self) {
        // Set NMIG
        {
            if self.get_nmip() && !self.get_nmig() {
                self.set_nmig(true);
            }
        }
        // Unset NMIG
        {
            if self.get_nmig() && self.get_brk_done() {
                self.set_nmig(false);
            }
        }
        // Set/Unset NMIL
        {
            if self.get_last_nmig() || self.get_last_nmil() {
                self.set_nmil(self.get_nmip());
            }

            self.set_last_nmil(self.get_nmil());
        }
    }

    fn nmi_phase_2(&mut self, io: &CpuIO) {
        // Set/Unset NMIP
        {
            self.set_nmip(!read_pin!(io.nmi));
        }
        // Set/Unset NMIL
        {
            self.set_last_nmig(self.get_nmig());
        }
    }

    fn rst_phase_1(&mut self) {
        // Set RESP
        {
            self.set_resp(!self.get_last_rst());
        }
        // Unset RESG
        {
            if self.get_resg() && self.get_brk_done() {
                self.set_resg(false);
            }
        }
    }

    fn rst_phase_2(&mut self, io: &CpuIO) {
        // Set RESP
        {
            self.set_last_rst(read_pin!(io.rst));
        }
        // Set RESG
        {
            if !self.get_resg() && self.get_resp() {
                self.set_resg(true);
            }
        }
    }

    fn irq_asserting(&self) -> bool {
        self.get_intg() || self.get_resg()
    }
}

mod pla;
