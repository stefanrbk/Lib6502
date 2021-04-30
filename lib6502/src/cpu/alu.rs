use super::*;

use bitfield::bitfield;

bitfield! {
    pub struct Alu(u128);
    pub u8, get_ai, set_ai: 0, 7;
    pub u8, get_bi, set_bi: 8, 15;
    pub u8, get_add, set_add: 16, 23;
    pub get_alu_c_out, set_alu_c_out: 24;
}

impl Alu {
    pub fn new() -> Alu {
        Alu { 0: 0 }
    }
}
