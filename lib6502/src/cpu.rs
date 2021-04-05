use super::*;
use std::{thread, thread::JoinHandle};

impl super::Cpu {
    pub fn new(io: CpuIO) -> Cpu {
        Cpu {
            pc: 0,
            s: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            io: io,
            nmi_state: NmiState::Clear,
            t_state: TState::Kil,
            rst_state: ResetState::Clear,
        }
    }

    pub fn start(mut self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.t_state = TState::T0;
            write_bus!(self.io.rw, true);

            while match self.t_state {
                TState::Kil => false,
                _ => true,
            } {
                self.cycle();
            }
            println!("Cpu.A = {:x}", self.a);
        })
    }

    fn cycle(&mut self) {
        self.phase_1();
        self.phase_2();
    }

    fn phase_1(&mut self) {
        self.io.phase_1_positive_edge.wait();
        // match self.t_state {
        //     _ =>
        // };
        if self.a == 0 {
            write_bus!(self.io.addr_bus, 0x0001);
            self.io.addr_stable.wait();
        } else {
            write_bus!(self.io.addr_bus, 0x0000);
            self.io.addr_stable.wait();
        }
        self.io.phase_1_negative_edge.wait();
    }
    fn phase_2(&mut self) {
        self.io.phase_2_positive_edge.wait();
        if self.a == 0 {
            self.io.data_stable.wait();
            self.a = read_bus!(self.io.data_bus);
            write_bus!(self.io.rw, false);
        } else {
            write_bus!(self.io.data_bus, 0x07);
            self.io.data_stable.wait();
            self.t_state = TState::Kil;
            write_bus!(self.io.rdy, true);
        }
        self.io.phase_2_negative_edge.wait();
    }
}

impl super::CpuIO {
    pub fn new() -> CpuIO {
        CpuIO {
            data_bus: new_bus!(0 as u8),
            addr_bus: new_bus!(0 as u16),
            rw: new_bus!(true),
            irq: new_bus!(false),
            rdy: new_bus!(false),
            nmi: new_bus!(false),
            rst: new_bus!(false),
            phase_1_negative_edge: null_barrier!(),
            phase_1_positive_edge: null_barrier!(),
            phase_2_negative_edge: null_barrier!(),
            phase_2_positive_edge: null_barrier!(),
            read_write_negative_edge: null_barrier!(),
            read_write_positive_edge: null_barrier!(),
            nmi_positive_edge: null_barrier!(),
            nmi_negative_edge: null_barrier!(),
            sync_negative_edge: null_barrier!(),
            sync_positive_edge: null_barrier!(),
            addr_stable: null_barrier!(),
            data_stable: null_barrier!(),
        }
    }
}
