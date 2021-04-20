use super::*;
use std::{thread, thread::JoinHandle};

impl super::Cpu {
    pub fn new(io: CpuIO) -> Cpu {
        Cpu {
            s: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            p: 0,
            pd: 0,
            ir: 0,
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
        self.irq_rst_control.phase_1();
        self.phase_1();
        self.io.phase_1_negative_edge.wait();
        self.io.phase_2_positive_edge.wait();
        self.irq_rst_control.phase_2(&mut self.p, &self.io);
        self.phase_2();
        self.io.phase_2_negative_edge.wait();
    }

    fn phase_1(&mut self) {}
    fn phase_2(&mut self) {}
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
        IrqRstControl {
            nmig: false,
            nmil: false,
            nmip: false,
            irqp: false,
            intg: false,
            resp: false,
            resg: false,
            last_rst: false,
            last_nmig: false,
            last_nmil: false,
            last_irq: false,
            brk_done: false,
        }
    }
    pub fn phase_1(&mut self) {
        self.rst_phase_1();
        self.nmi_phase_1();
        self.irq_phase_1();
    }

    pub fn phase_2(&mut self, p: &mut u8, io: &CpuIO) {
        self.rst_phase_2(&io);
        self.nmi_phase_2(&io);
        self.irq_phase_2(&io);
        self.intg_phase_2(p);

        *p = *p | 16; // Set b flag
    }

    fn intg_phase_2(&mut self, p: &u8) {
        // Set INTG
        {
            if (self.irqp && ((p & (1 << 2)) != 0)) || self.nmip {
                self.intg = true;
            }
        }
        // Reset INTG
        {
            if self.intg && self.brk_done {
                self.intg = false;
            }
        }
    }

    fn irq_phase_1(&mut self) {
        // Set IRQP
        {
            self.irqp = self.last_irq;
        }
    }

    fn irq_phase_2(&mut self, io: &CpuIO) {
        // Set IRQP
        {
            self.last_irq = !read_pin!(io.irq);
        }
    }

    fn nmi_phase_1(&mut self) {
        // Set NMIG
        {
            if self.nmip && !self.nmig {
                self.nmig = true;
            }
        }
        // Reset NMIG
        {
            if self.nmig && self.brk_done {
                self.nmig = false;
            }
        }
        // Set/Reset NMIL
        {
            if self.last_nmig || self.last_nmil {
                self.nmil = self.nmip;
            }

            self.last_nmil = self.nmil;
        }
    }

    fn nmi_phase_2(&mut self, io: &CpuIO) {
        // Set/Reset NMIP
        {
            self.nmip = !read_pin!(io.nmi);
        }
        // Set/Reset NMIL
        {
            self.last_nmig = self.nmig;
        }
    }

    fn rst_phase_1(&mut self) {
        // Set RESP
        {
            self.resp = !self.last_rst;
        }
        // Reset RESG
        {
            if self.resg && self.brk_done {
                self.resg = false;
            }
        }
    }

    fn rst_phase_2(&mut self, io: &CpuIO) {
        // Set RESP
        {
            self.last_rst = read_pin!(io.rst);
        }
        // Set RESG
        {
            if !self.resg && self.resp {
                self.resg = true;
            }
        }
    }

    fn irq_asserting(&mut self) -> bool {
        self.intg || self.resg
    }
}

mod pla;
