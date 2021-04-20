use std::sync::{Arc, RwLock};

mod clock;
mod cpu;
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

struct IrqRstControl {
    nmig: bool,
    nmil: bool,
    nmip: bool,
    irqp: bool,
    intg: bool,
    resp: bool,
    resg: bool,
    last_rst: bool,
    last_nmig: bool,
    last_nmil: bool,
    last_irq: bool,
    brk_done: bool,
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
    pub db: Bus<u8>,
    pub abh: Bus<u8>,
    pub abl: Bus<u8>,
    pub rw: DataPin,
    pub irq: DataPin,
    pub rdy: DataPin,
    pub nmi: DataPin,
    pub rst: DataPin,
    pub sync: DataPin,
    pub so: DataPin,
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
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    pd: u8,
    ir: u8,
    dor: u8,
    dl: u8,
    pcls: u8,
    pcl: u8,
    ai: u8,
    bi: u8,
    add: u8,
    abl: u8,
    abh: u8,
    pchs: u8,
    pch: u8,
    db: u8,
    adl: u8,
    adh: u8,
    sb: u8,
    irq_rst_control: IrqRstControl,
    t_state: TState,
    io: CpuIO,
}
