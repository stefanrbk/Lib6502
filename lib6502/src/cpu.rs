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

    pub fn start(self) -> JoinHandle<()> {
        thread::spawn(move || {
            while match self.t_state {
                TState::Kil => false,
                _ => true,
            } {
                self.cycle();
            }
        })
    }

    fn cycle(&self) {
        self.io.phase_1_positive_edge.wait();
        self.io.phase_1_negative_edge.wait();
        self.io.phase_2_positive_edge.wait();
        self.io.phase_2_negative_edge.wait();
    }
}

impl super::CpuIO {
    pub fn new() -> CpuIO {
        CpuIO {
            data_bus: Bus::new(0 as u8),
            addr_bus: Bus::new(0 as u16),
            rw: Bus::new(true),
            irq: Bus::new(false),
            rdy: Bus::new(false),
            nmi: Bus::new(false),
            rst: Bus::new(false),
            phase_1_negative_edge: no_barrier!(),
            phase_1_positive_edge: no_barrier!(),
            phase_2_negative_edge: no_barrier!(),
            phase_2_positive_edge: no_barrier!(),
            read_write_negative_edge: no_barrier!(),
            read_write_positive_edge: no_barrier!(),
            nmi_positive_edge: no_barrier!(),
            nmi_negative_edge: no_barrier!(),
            sync_negative_edge: no_barrier!(),
            sync_positive_edge: no_barrier!(),
        }
    }
}
