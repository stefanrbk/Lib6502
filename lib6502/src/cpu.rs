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
            nmi_state: new_bus!(NmiState::Clear),
            t_state: TState::Kil,
            rst_state: new_pin!(Clear),
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

    pub fn trigger_irq(&mut self) {
        write_bus!(self.io.irq_count, read_bus!(self.io.irq_count) + 1);
        clear_pin!(self.io.irq);
    }

    pub fn clear_irq(&mut self) {
        write_bus!(self.io.irq_count, read_bus!(self.io.irq_count) + 1);
        if read_bus!(self.io.irq_count) == 0 {
            set_pin!(self.io.irq);
        }
    }

    fn cycle(&mut self) {
        self.io.phase_1_positive_edge.wait();
        self.handle_nmi();
        self.phase_1();
        self.io.phase_1_negative_edge.wait();
        self.io.phase_2_positive_edge.wait();
        self.phase_2();
        self.io.phase_2_negative_edge.wait();
    }

    fn handle_nmi(&mut self) {
        match read_pin!(self.io.nmi) {
            Pin::Clear => match read_bus!(self.nmi_state) {
                NmiState::Clear => write_bus!(self.nmi_state, NmiState::Set1),
                NmiState::Set1 => write_bus!(self.nmi_state, NmiState::Set2),
                NmiState::Set2 => write_bus!(self.nmi_state, NmiState::SetRecognized),
                NmiState::SetRecognized => {}
            },
            Pin::Set => match read_bus!(self.nmi_state) {
                NmiState::Set1 | NmiState::Set2 => {
                    write_bus!(self.nmi_state, NmiState::Clear)
                }
                _ => {}
            },
        }
    }

    fn phase_1(&mut self) {
        match self.t_state {
            TState::T0 => {
                set_pin!(self.io.sync);
                self.io.sync_positive_edge.wait();
                match read_pin!(self.nmi_state) {
                    NmiState::SetRecognized => {
                        // acknowledge non-maskable interrupt
                        write_bus!(self.nmi_state, NmiState::Clear);

                        // TODO: logic to start non-maskable interrupt

                        return;
                    }
                    _ => {}
                }
                match read_pin!(self.io.irq) {
                    Pin::Clear => {
                        // TODO: logic to start interrupt
                        return;
                    }
                    _ => {}
                }
            }
            _ => {}
        };
    }
    fn phase_2(&mut self) {
        if self.a == 0 {
            self.io.data_stable.wait();
            self.a = read_bus!(self.io.data_bus);
            clear_pin!(self.io.rw);
        } else {
            write_bus!(self.io.data_bus, 0x07);
            self.io.data_stable.wait();
            self.t_state = TState::Kil;
            set_pin!(self.io.rdy);
        }
    }
}

impl super::CpuIO {
    pub fn new() -> CpuIO {
        CpuIO {
            data_bus: new_bus!(0 as u8),
            addr_bus: new_bus!(0 as u16),
            rw: new_pin!(Set),
            irq: new_pin!(Set),
            irq_count: new_bus!(0 as u8),
            rdy: new_pin!(Clear),
            nmi: new_pin!(Set),
            rst: new_pin!(Clear),
            sync: new_pin!(Clear),
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
