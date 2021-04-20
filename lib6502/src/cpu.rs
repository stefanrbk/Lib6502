use super::*;
use std::{thread, thread::JoinHandle};

impl super::Cpu {
    pub fn get_p(self) -> u8 {
        self._p.0
    }

    pub fn set_p(&mut self, value: u8) {
        self._p.0 = value & 0b11011111;
    }

    flag_set_unset!(c);
    flag_set_unset!(z);
    flag_set_unset!(i);
    flag_set_unset!(d);
    flag_set_unset!(b);
    flag_set_unset!(v);
    flag_set_unset!(n);

    pub fn new(io: CpuIO) -> Cpu {
        Cpu {
            s: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            _p: StatusFlags { 0: 0 },
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
        self.irq_rst_control.phase_2(&mut self._p, &self.io);
        self.phase_2();
        self.io.phase_2_negative_edge.wait();
    }

    fn phase_1(&mut self) {}
    fn phase_2(&mut self) {}
}

impl super::StatusFlags {
    bitfield_set_unset!(c);
    bitfield_set_unset!(z);
    bitfield_set_unset!(i);
    bitfield_set_unset!(d);
    bitfield_set_unset!(b);
    bitfield_set_unset!(v);
    bitfield_set_unset!(n);
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

    bitfield_set_unset!(nmig);
    bitfield_set_unset!(nmil);
    bitfield_set_unset!(nmip);
    bitfield_set_unset!(irqp);
    bitfield_set_unset!(intg);
    bitfield_set_unset!(resp);
    bitfield_set_unset!(resg);
    bitfield_set_unset!(last_rst);
    bitfield_set_unset!(last_nmig);
    bitfield_set_unset!(last_nmil);
    bitfield_set_unset!(last_irq);
    bitfield_set_unset!(brk_done);

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

        p.set_b(); // Set b flag
    }

    fn intg_phase_2(&mut self, p: &StatusFlags) {
        // Set INTG
        {
            if (self.get_irqp() && p.get_i()) || self.get_nmip() {
                self.set_intg();
            }
        }
        // Unset INTG
        {
            if self.get_intg() && self.get_brk_done() {
                self.unset_intg();
            }
        }
    }

    fn irq_phase_1(&mut self) {
        // Set IRQP
        {
            self.set_irqp_value(self.get_last_irq());
        }
    }

    fn irq_phase_2(&mut self, io: &CpuIO) {
        // Set IRQP
        {
            self.set_last_irq_value(!read_pin!(io.irq));
        }
    }

    fn nmi_phase_1(&mut self) {
        // Set NMIG
        {
            if self.get_nmip() && !self.get_nmig() {
                self.set_nmig();
            }
        }
        // Unset NMIG
        {
            if self.get_nmig() && self.get_brk_done() {
                self.unset_nmig();
            }
        }
        // Set/Unset NMIL
        {
            if self.get_last_nmig() || self.get_last_nmil() {
                self.set_nmil_value(self.get_nmip());
            }

            self.set_last_nmil_value(self.get_nmil());
        }
    }

    fn nmi_phase_2(&mut self, io: &CpuIO) {
        // Set/Unset NMIP
        {
            self.set_nmip_value(!read_pin!(io.nmi));
        }
        // Set/Unset NMIL
        {
            self.set_last_nmig_value(self.get_nmig());
        }
    }

    fn rst_phase_1(&mut self) {
        // Set RESP
        {
            self.set_resp_value(!self.get_last_rst());
        }
        // Unset RESG
        {
            if self.get_resg() && self.get_brk_done() {
                self.unset_resg();
            }
        }
    }

    fn rst_phase_2(&mut self, io: &CpuIO) {
        // Set RESP
        {
            self.set_last_rst_value(read_pin!(io.rst));
        }
        // Set RESG
        {
            if !self.get_resg() && self.get_resp() {
                self.set_resg();
            }
        }
    }

    fn irq_asserting(&mut self) -> bool {
        self.get_intg() || self.get_resg()
    }
}

mod pla;
