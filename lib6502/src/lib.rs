use bitfield::bitfield;
use std::sync::{Arc, RwLock};

pub mod clock;
pub mod cpu;
mod memory;

pub type Action = fn();
pub type Bus<T> = Arc<RwLock<T>>;
pub type DataPin = Bus<bool>;
pub type Barrier = Arc<std::sync::Barrier>;

const MAXMEM: usize = 1024 * 64;

#[macro_export]
macro_rules! cond {
    ($val:expr, $t:expr, $f:expr) => {
        if $val {
            $t
        } else {
            $f
        }
    };
}

#[macro_export]
macro_rules! null_barrier {
    () => {
        new_barrier!(1);
    };
}
#[macro_export]
macro_rules! new_bus {
    ($value:expr) => {
        std::sync::Arc::new(std::sync::RwLock::new($value))
    };
}
#[macro_export]
macro_rules! new_pin_unset {
    () => {
        new_bus!(false)
    };
}
#[macro_export]
macro_rules! new_pin_set {
    () => {
        new_bus!(true)
    };
}
#[macro_export]
macro_rules! new_barrier {
    ($value:expr) => {
        std::sync::Arc::new(std::sync::Barrier::new($value))
    };
}
#[macro_export]
macro_rules! write_bus {
    ($bus:expr, $value:expr) => {
        (*$bus.write().unwrap()) = $value
    };
    ($bus:expr, $index:expr, $value:expr) => {
        (*$bus.write().unwrap())[$index] = $value
    };
}
#[macro_export]
macro_rules! read_bus {
    ($bus:expr) => {
        (*$bus.read().unwrap())
    };
}
#[macro_export]
macro_rules! read_pin {
    ($pin:expr) => {
        read_bus!($pin)
    };
}
#[macro_export]
macro_rules! set_pin {
    ($pin:expr) => {
        (*$pin.write().unwrap()) = true
    };
}
#[macro_export]
macro_rules! clear_pin {
    ($pin:expr) => {
        (*$pin.write().unwrap()) = false
    };
}

//StatusFlags
bitfield! {
    struct StatusFlags(u8);
    get_c, set_c: 0;
    get_z, set_z: 1;
    get_i, set_i: 2;
    get_d, set_d: 3;
    get_b, set_b: 4;
    get_v, set_v: 6;
    get_n, set_n: 7;
}

// TimingControl
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

// ReadyControl
bitfield! {
    struct ReadyControl(u8);
    pub get_not_rdy_last_phase_2, set_not_rdy_last_phase_2: 0;
    pub get_hold_branch, set_hold_branch: 1;
    pub get_not_rdy, set_not_rdy: 2;
    pub get_rdy_last_phase_1, set_rdy_last_phase_1: 3;
    pub get_not_rdy_delay, set_not_rdy_delay: 4;
}

// LogicControl
bitfield! {
    struct LogicControl(u64);
}

// Alu
bitfield! {
    struct Alu(u128);
    u8, get_ai, set_ai: 0, 7;
    u8, get_bi, set_bi: 8, 15;
    u8, get_add, set_add: 16, 23;
    get_alu_c_out, set_alu_c_out: 24;
}

pub struct Clock {
    pub phase_1_positive_edge: Barrier,
    pub phase_1_negative_edge: Barrier,
    pub phase_2_positive_edge: Barrier,
    pub phase_2_negative_edge: Barrier,
    pub p1: DataPin,
    pub p2: DataPin,
    pub kill: Bus<bool>,
}

pub struct Memory {
    data: [u8; MAXMEM],
}
