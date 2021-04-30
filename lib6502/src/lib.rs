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
