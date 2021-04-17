use std::sync::{Arc, RwLock};

mod clock;
mod cpu;
mod memory;

pub type Action = fn();
pub type Bus<T> = Arc<RwLock<T>>;
pub type DataPin = Bus<Pin>;
pub type Barrier = Arc<std::sync::Barrier>;

const MAXMEM: usize = 1024 * 64;

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
macro_rules! new_pin {
    ($value:ident) => {
        new_bus!(Pin::$value)
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
        (*$pin.write().unwrap()) = Pin::Set
    };
}
#[macro_export]
macro_rules! clear_pin {
    ($pin:expr) => {
        (*$pin.write().unwrap()) = Pin::Clear
    };
}

enum TState {
    T0,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
    Kil,
}

enum NmiState {
    Clear,
    Set1,
    Set2,
    SetRecognized,
}

pub enum Pin {
    Clear,
    Set,
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

pub struct CpuIO {
    pub data_bus: Bus<u8>,
    pub addr_bus: Bus<u16>,
    pub rw: DataPin,
    pub irq: DataPin,
    pub irq_count: Bus<u8>,
    pub rdy: DataPin,
    pub nmi: DataPin,
    pub rst: DataPin,
    pub sync: DataPin,
    pub phase_1_positive_edge: Barrier,
    pub phase_2_positive_edge: Barrier,
    pub phase_1_negative_edge: Barrier,
    pub phase_2_negative_edge: Barrier,
    pub read_write_positive_edge: Barrier,
    pub read_write_negative_edge: Barrier,
    pub sync_positive_edge: Barrier,
    pub sync_negative_edge: Barrier,
    pub addr_stable: Barrier,
    pub data_stable: Barrier,
}

pub struct Cpu {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    nmi_state: Bus<NmiState>,
    t_state: TState,
    rst_state: DataPin,
    io: CpuIO,
}
