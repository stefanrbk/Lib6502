use super::*;

use bitfield::bitfield;

bitfield! {
    pub struct Decoder(u128);
    pub t3_branch, _: 0;
}

impl Decoder {
    pub fn new(ir: u8, tstate: &TimingControl) -> Decoder {
        let mut value = 0 as u128;

        for i in (0..=127).rev() {
            if pla::check_opcode(ir, tstate.get_tstate(), pla::PLA[i]) {
                value |= 1;
            }
            value <<= 1;
        }

        Decoder { 0: value }
    }
}
