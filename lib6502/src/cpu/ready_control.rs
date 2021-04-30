use super::*;

use bitfield::bitfield;

bitfield! {
    pub struct ReadyControl(u8);
    pub get_not_rdy_last_phase_2, set_not_rdy_last_phase_2: 0;
    pub get_hold_branch, set_hold_branch: 1;
    pub get_not_rdy, set_not_rdy: 2;
    pub get_rdy_last_phase_1, set_rdy_last_phase_1: 3;
    pub get_not_rdy_delay, set_not_rdy_delay: 4;
}

impl ReadyControl {
    pub fn new() -> ReadyControl {
        ReadyControl { 0: 0 }
    }
    pub fn phase_1(&mut self, io: &CpuIO) {
        let rdy = read_pin!(io.rdy);
        self.set_not_rdy(!rdy);

        self.set_hold_branch(self.get_not_rdy_last_phase_2());
        self.set_rdy_last_phase_1(rdy);
    }
    pub fn phase_2(&mut self, io: &CpuIO) {
        let rdy = read_pin!(io.rdy);
        self.set_not_rdy(!rdy);

        self.set_not_rdy_last_phase_2(!rdy);
        self.set_not_rdy_delay(!self.get_rdy_last_phase_1());
    }
}
