use super::*;

pub struct Registers {
    pub s: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: StatusFlags,
    pub ir: u8,
    pub dor: u8,
    pub dl: u8,
    pub abh: u8,
    pub abl: u8,
    pub pchs: u8,
    pub pch: u8,
    pub pcls: u8,
    pub pcl: u8,
    pub ai: u8,
    pub bi: u8,
    pub alu_result: AluResult,
}

//StatusFlags
bitfield! {
    #[derive(Copy, Clone)]
    pub struct StatusFlags(u8);
    pub get_c, set_c: 0;
    pub get_z, set_z: 1;
    pub get_i, set_i: 2;
    pub get_d, set_d: 3;
    pub get_b, set_b: 4;
    pub get_v, set_v: 6;
    pub get_n, set_n: 7;
}

//AluResult
bitfield! {
    #[derive(Copy, Clone)]
    pub struct AluResult(u16);
    pub u8, get_add, set_add: 0, 7;
    pub get_avr, set_avr: 8;
    pub get_acr, set_acr: 9;
    pub get_hc, set_hc: 10;
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            s: 0xFF,
            a: 0,
            x: 0,
            y: 0,
            p: StatusFlags { 0: 0 },
            ir: 0,
            dor: 0,
            dl: 0,
            pcls: 0,
            pcl: 0,
            abh: 0,
            pchs: 0,
            pch: 0,
            abl: 0,
            alu_result: AluResult { 0: 0 },
            ai: 0,
            bi: 0,
        }
    }
    pub fn get_p(&self) -> u8 {
        self.p.0
    }

    pub fn get_flags(&self) -> StatusFlags {
        self.p
    }

    pub fn get_flags_ref(&mut self) -> &StatusFlags {
        &self.p
    }

    pub fn set_p(&mut self, value: u8) {
        self.p.0 = value & 0b11011111;
    }
}
