use bitfield::bitfield;

use super::*;

bitfield! {
    pub struct Predecoder(u32);
    pub u8, get_pd, set_pd: 0, 7;
    pub get_two_cycle, set_two_cycle: 16;
    pub get_one_byte, set_one_byte: 17;
}
impl Predecoder {
    pub fn new() -> Predecoder {
        Predecoder { 0: 0 }
    }

    pub fn phase_1(&mut self, ir: &mut u8, timing: &TimingControl) {
        if timing.get_fetch() {
            *ir = self.get_pd();
        }
    }

    pub fn phase_2(&mut self, dl: u8, ir: &mut u8, timing: &TimingControl, irq: &IrqRstControl) {
        self.set_pd(dl);
        if timing.get_fetch() && !irq.irq_asserting() {
            self.clear_ir(ir);
        } else {
            let pd_0xx0xx0x = pla::check_opcode(self.get_pd(), 0, pla::PD0XX0XX0X);
            let pd_1xx000x0 = pla::check_opcode(self.get_pd(), 0, pla::PD1XX000X0);
            let pd_xxx010x1 = pla::check_opcode(self.get_pd(), 0, pla::PDXXX010X1);
            let pd_xxxx10x0 = pla::check_opcode(self.get_pd(), 0, pla::PDXXXX10X0);

            self.set_two_cycle(pd_xxx010x1 || pd_1xx000x0 || (pd_xxxx10x0 && !pd_0xx0xx0x));
            self.set_one_byte(pd_xxxx10x0);
        }
    }
    pub fn clear_ir(&mut self, ir: &mut u8) {
        *ir = 0;
        self.set_two_cycle(false);
        self.set_one_byte(false);
    }
}
