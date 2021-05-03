use super::*;

use alu::Alu;
use cpu_io::CpuIO;
use data_path_control::DataPathControl;
use decoder::Decoder;
use irq_rst::IrqRstControl;
use predecoder::Predecoder;
use ready_control::ReadyControl;
use registers::Registers;
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
mod registers;
mod timing_control;

pub struct Cpu {
    registers: Registers,
    irq_rst_control: IrqRstControl,
    ready_control: ReadyControl,
    predecoder: Predecoder,
    decoder: Decoder,
    timing_control: TimingControl,
    dp_control: DataPathControl,
    alu: Alu,
    io: CpuIO,
}

impl Cpu {
    #[export_name = "new_cpu"]
    pub fn new(io: CpuIO) -> Cpu {
        Cpu {
            registers: Registers::new(),
            irq_rst_control: IrqRstControl::new(),
            ready_control: ReadyControl::new(),
            predecoder: Predecoder::new(),
            decoder: Decoder { 0: 0 },
            timing_control: TimingControl::new(),
            dp_control: DataPathControl::new(),
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
        self.predecoder
            .phase_1(&mut self.registers.ir, &self.timing_control);
        self.decoder = Decoder::new(self.registers.ir, &self.timing_control);
    }
    fn phase_2(&mut self) {
        // Update input data latch
        self.registers.dl = read_bus!(self.io.data_bus);

        self.ready_control.phase_2(&self.io);
        self.irq_rst_control
            .phase_2(&mut self.registers.p, &self.io);
        self.timing_control
            .phase_2(&self.irq_rst_control, &self.ready_control);
        // Update predecoder
        self.predecoder.set_pd(self.registers.dl);

        self.predecoder.phase_2(
            self.registers.dl,
            &mut self.registers.ir,
            &self.timing_control,
            &self.irq_rst_control,
        );
    }
}
