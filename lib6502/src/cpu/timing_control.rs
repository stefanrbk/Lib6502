use super::*;

bitfield! {
    pub struct TimingControl(u8);
    pub get_fetch, set_fetch: 0;
    pub get_sync, set_sync: 1;
    pub get_sync_last_phase_2, set_sync_last_phase_2: 2;
    pub get_a, set_a: 3;
    pub get_b, set_b: 4;
    pub get_c, set_c: 5;
    pub get_unk_20, set_unk_20: 6;
    pub get_branch_back_phase_1, set_branch_back_phase_1: 8;
    pub get_unk_11, set_unk_11: 9;
}

impl super::TimingControl {
    pub fn new() -> TimingControl {
        TimingControl { 0: 0 }
    }
    pub fn get_tstate(&self) -> u8 {
        0
    }
    pub fn phase_1(
        &mut self,
        io: &mut CpuIO,
        alu: &Alu,
        decoder: &Decoder,
        rst: &IrqRstControl,
        rc: &ReadyControl,
    ) {
        // c
        {
            let c = !(self.short_circuit_branch_add(alu, decoder, rc) || self.get_unk_20());
            self.set_c(c);
        }
        // a
        {
            self.set_a(!(rc.get_not_rdy() || self.get_c()));
        }

        // Sync
        {
            if self.not_sync(alu, decoder, rc) {
                clear_pin!(io.sync);
            } else {
                set_pin!(io.sync);
            }
        }
    }
    pub fn phase_2(&mut self, rst: &IrqRstControl, rc: &ReadyControl) {
        // Fetch
        {
            self.set_fetch(!rc.get_not_rdy() && self.get_sync());
        }
        self.set_sync_last_phase_2(self.get_sync());
    }
    fn not_sync(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !self.sync_left(alu, decoder, rc) && !self.get_b()
    }
    fn sync_left(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !rc.get_not_rdy() && !self.not_rdy_lower(alu, decoder, rc)
    }
    fn not_rdy_lower(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !self.short_circuit_branch_add(alu, decoder, rc) && !self.get_unk_20()
    }
    fn short_circuit_branch_add(&self, alu: &Alu, decoder: &Decoder, rc: &ReadyControl) -> bool {
        self.not_t3_branch_or_not_rdy_delay(decoder, rc)
            && (self.get_branch_back_phase_1() != alu.get_alu_c_out())
    }
    fn not_t3_branch_or_not_rdy_delay(&self, decoder: &Decoder, rc: &ReadyControl) -> bool {
        !(decoder.get_t3_branch() || rc.get_not_rdy_delay())
    }
}
