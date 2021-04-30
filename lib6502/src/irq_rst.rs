use super::*;
use crate::CpuIO;
use bitfield::bitfield;

bitfield! {
    pub struct IrqRstControl(u16);
    get_nmig, set_nmig: 0;
    get_nmil, set_nmil: 1;
    get_nmip, set_nmip: 2;
    get_irqp, set_irqp: 3;
    get_intg, set_intg: 4;
    get_resp, set_resp: 5;
    get_resg, set_resg: 6;
    get_last_rst, set_last_rst: 7;
    get_last_nmig, set_last_nmig: 8;
    get_last_nmil, set_last_nmil: 9;
    get_last_irq, set_last_irq: 10;
    get_brk_done, set_brk_done: 11;
}

impl IrqRstControl {
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

    pub fn irq_asserting(&self) -> bool {
        self.get_intg() || self.get_resg()
    }
}
