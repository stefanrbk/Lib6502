use super::*;

use alu::Alu;
use cpu_io::CpuIO;
use data_path_control::DataPathControl;
use decoder::Decoder;
use irq_rst::IrqRstControl;
use pla::Pla;
use predecoder::Predecoder;
use ready_control::ReadyControl;
use std::{thread, thread::JoinHandle};
use timing_control::TimingControl;

mod alu;
pub mod cpu_io;
mod data_path_control;
mod decoder;
mod irq_rst;
mod pla;
mod predecoder;
mod ready_control;
mod timing_control;

//StatusFlags
bitfield! {
    struct StatusFlags(u8);
    get_c, set_c: 0;
    get_z, set_z: 1;
    get_i, set_i: 2;
    get_d, set_d: 3;
    get_b, set_b: 4;
    get_v, set_v: 6;
    get_n, set_n: 7;
}

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
