use super::*;
use bitfield::bitfield;

type DpAction = fn(&mut DataPathControl, &mut Registers);

pub struct DataPathControl {
    adh: u8,
    adl: u8,
    db: u8,
    sb: u8,
    alu_flags: AluFlags,
    adh_bus_write: Vec<DpAction>,
    adh_bus_modify: Vec<DpAction>,
    adh_bus_read: Vec<DpAction>,
    adl_bus_write: Vec<DpAction>,
    adl_bus_modify: Vec<DpAction>,
    adl_bus_read: Vec<DpAction>,
    db_bus_write: Vec<DpAction>,
    db_bus_read: Vec<DpAction>,
    sb_bus_write: Vec<DpAction>,
    sb_bus_read: Vec<DpAction>,
}

bitfield! {
    pub struct AluFlags(u8);
    sums, set_sums: 0;
    ands, set_ands: 1;
    eors, set_eors: 2;
    ors, set_ors: 3;
    srs, set_srs: 4;
    c_in, set_c_in: 5;
    d_in, set_d_in: 6;
    d_sub_in, set_d_sub_in: 7;
}

impl DataPathControl {
    pub fn new() -> DataPathControl {
        DataPathControl {
            adh: 0,
            adl: 0,
            db: 0,
            sb: 0,
            alu_flags: AluFlags { 0: 0 },
            adh_bus_read: Vec::new(),
            adl_bus_read: Vec::new(),
            db_bus_read: Vec::new(),
            sb_bus_read: Vec::new(),
            adh_bus_modify: Vec::new(),
            adl_bus_modify: Vec::new(),
            adh_bus_write: Vec::new(),
            adl_bus_write: Vec::new(),
            db_bus_write: Vec::new(),
            sb_bus_write: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.adh_bus_read = Vec::new();
        self.adh_bus_modify = Vec::new();
        self.adh_bus_write = Vec::new();
        self.adl_bus_read = Vec::new();
        self.adl_bus_modify = Vec::new();
        self.adl_bus_write = Vec::new();
        self.db_bus_read = Vec::new();
        self.db_bus_write = Vec::new();
        self.sb_bus_read = Vec::new();
        self.sb_bus_write = Vec::new();
    }

    fn dl_to_db(&mut self, reg: &mut Registers) {
        self.db = reg.dl;
    }

    fn dl_to_adl(&mut self, reg: &mut Registers) {
        self.adl = reg.dl;
    }

    fn dl_to_adh(&mut self, reg: &mut Registers) {
        self.adh = reg.dl;
    }

    fn _0_to_adh_0(&mut self, _reg: &mut Registers) {
        self.adh &= 0b11111110;
    }

    fn _0_to_adh_1_7(&mut self, _reg: &mut Registers) {
        self.adh &= 0b00000001;
    }

    fn adh_to_abh(&mut self, reg: &mut Registers) {
        reg.abh = self.adh;
    }

    fn adl_to_abl(&mut self, reg: &mut Registers) {
        reg.abl = self.adl;
    }

    fn pcl_to_pcl(&mut self, reg: &mut Registers) {
        reg.pcls = reg.pcl;
    }

    fn adl_to_pcl(&mut self, reg: &mut Registers) {
        reg.pcls = self.adl;
    }

    fn inc_pc(&mut self, reg: &mut Registers) {
        reg.pcls = u8::wrapping_add(reg.pcls, 1);
        if reg.pcls == 0 {
            reg.pchs = u8::wrapping_add(reg.pchs, 1);
        }
    }

    fn pcl_to_db(&mut self, reg: &mut Registers) {
        self.db = reg.pcl;
    }

    fn pcl_to_adl(&mut self, reg: &mut Registers) {
        self.adl = reg.pcl;
    }

    fn pch_to_pch(&mut self, reg: &mut Registers) {
        reg.pchs = reg.pch;
    }

    fn adh_to_pch(&mut self, reg: &mut Registers) {
        reg.pchs = self.adh;
    }

    fn pch_to_db(&mut self, reg: &mut Registers) {
        self.db = reg.pch;
    }

    fn pch_to_adh(&mut self, reg: &mut Registers) {
        self.adh = reg.pch;
    }

    fn sb_to_adh(&mut self, _reg: &mut Registers) {
        self.adh = self.sb;
    }

    fn sb_to_db(&mut self, _reg: &mut Registers) {
        self.db = self.sb;
    }

    fn _0_to_adl_0(&mut self, _reg: &mut Registers) {
        self.adl &= 0b11111110;
    }

    fn _0_to_adl_1(&mut self, _reg: &mut Registers) {
        self.adl &= 0b11111101;
    }

    fn _0_to_adl_2(&mut self, _reg: &mut Registers) {
        self.adl &= 0b11111011;
    }

    fn s_to_adl(&mut self, reg: &mut Registers) {
        self.adl = reg.s;
    }

    fn sb_to_s(&mut self, reg: &mut Registers) {
        reg.s = self.sb;
    }

    //? Do I need a fn s_to_s?

    fn s_to_sb(&mut self, reg: &mut Registers) {
        self.sb = reg.s;
    }

    fn ndb_to_add(&mut self, reg: &mut Registers) {
        reg.bi = !self.db;
    }

    fn db_to_add(&mut self, reg: &mut Registers) {
        reg.bi = self.db;
    }

    fn adl_to_add(&mut self, reg: &mut Registers) {
        reg.ai = self.adl;
    }

    fn alu_c_in(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_c_in(true);
    }

    fn alu_d_in(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_d_in(true);
    }

    fn alu_d_sub_in(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_d_sub_in(true);
    }

    fn sums(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_sums(true);
    }

    fn ands(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_ands(true);
    }

    fn eors(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_eors(true);
    }

    fn ors(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_ors(true);
    }

    fn srs(&mut self, _reg: &mut Registers) {
        self.alu_flags.set_srs(true);
    }

    fn add_to_adl(&mut self, reg: &mut Registers) {
        self.adl = reg.alu_result.get_add();
    }

    fn add_to_sb_0_6(&mut self, reg: &mut Registers) {
        self.sb |= reg.alu_result.get_add() & 0b01111111;
    }

    fn add_to_sb_7(&mut self, reg: &mut Registers) {
        self.sb |= reg.alu_result.get_add() & 0b10000000;
    }

    fn _0_to_add(&mut self, reg: &mut Registers) {
        reg.ai = 0;
    }

    fn sb_to_add(&mut self, reg: &mut Registers) {
        reg.ai = self.sb;
    }

    fn sb_to_a(&mut self, reg: &mut Registers) {
        reg.a = u8::wrapping_add(self.sb, self.decimal_adjust(reg.ai, reg.bi));
    }

    fn a_to_db(&mut self, reg: &mut Registers) {
        self.db = reg.a;
    }

    fn a_to_sb(&mut self, reg: &mut Registers) {
        self.sb = reg.a;
    }

    fn sb_to_x(&mut self, reg: &mut Registers) {
        reg.x = self.sb;
    }

    fn x_to_sb(&mut self, reg: &mut Registers) {
        self.sb = reg.x;
    }

    fn sb_to_y(&mut self, reg: &mut Registers) {
        reg.y = self.sb;
    }

    fn y_to_sb(&mut self, reg: &mut Registers) {
        self.sb = reg.y;
    }

    fn p_to_db(&mut self, reg: &mut Registers) {
        self.db = reg.get_p();
    }

    fn db_0_to_c(&mut self, reg: &mut Registers) {
        reg.p.set_c(self.db & 0b00000001 != 0);
    }

    fn ir_5_to_c(&mut self, reg: &mut Registers) {
        reg.p.set_c(reg.ir & 0b00100000 != 0);
    }

    fn acr_to_c(&mut self, reg: &mut Registers) {
        reg.p.set_c(reg.alu_result.get_acr());
    }

    fn db_1_to_z(&mut self, reg: &mut Registers) {
        reg.p.set_z(self.db & 0b00000010 != 0);
    }

    fn dbz_to_z(&mut self, reg: &mut Registers) {
        reg.p.set_z(self.db == 0);
    }

    fn db_2_to_i(&mut self, reg: &mut Registers) {
        reg.p.set_i(self.db & 0b00000100 != 0);
    }

    fn ir_5_to_i(&mut self, reg: &mut Registers) {
        reg.p.set_i(reg.ir & 0b00100000 != 0);
    }

    fn db_3_to_d(&mut self, reg: &mut Registers) {
        reg.p.set_d(self.db & 0b00001000 != 0);
    }

    fn ir_5_to_d(&mut self, reg: &mut Registers) {
        reg.p.set_d(reg.ir & 0b00100000 != 0);
    }

    fn db_6_to_v(&mut self, reg: &mut Registers) {
        reg.p.set_v(self.db & 0b01000000 != 0);
    }

    fn avr_to_v(&mut self, reg: &mut Registers) {
        reg.p.set_v(reg.alu_result.get_avr());
    }

    fn set_v(&mut self, reg: &mut Registers) {
        reg.p.set_v(true);
    }

    fn db_7_to_n(&mut self, reg: &mut Registers) {
        reg.p.set_n(self.db & 0b10000000 != 0);
    }

    fn decimal_adjust(&mut self, ai: u8, bi: u8) -> u8 {
        let mut adj = 0 as u8;
        let hc = ((ai & 0x0F) + (bi & 0x0F)) > 0x0F;
        let c = u16::wrapping_add((ai & 0xF0) as u16, (bi & 0xF0) as u16) > 0xF0;
        if self.alu_flags.d_sub_in() && !hc {
            adj |= 0b00001010;
        }
        if self.alu_flags.d_in() && hc {
            adj |= 0b00000110;
        }
        if self.alu_flags.d_sub_in() && !c {
            adj |= 0b10100000;
        }
        if self.alu_flags.d_in() && c {
            adj |= 0b01100000;
        }
        adj
    }
}
