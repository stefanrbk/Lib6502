use bitfield::bitfield;
use paste::paste;
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
macro_rules! bitfield_set_unset {
    ($name:ident) => {
        paste! {
            pub fn [<set_ $name>](&mut self) {
                self.[<set_ $name _value>](true);
            }
            pub fn [<unset_ $name>](&mut self) {
                self.[<set_ $name _value>](false);
            }
        }
    };
}

#[macro_export]
macro_rules! flag_set_unset {
    ($name:ident) => {
        paste! {
            pub fn [<set_ $name>](&mut self) {
                self._p.[<set_ $name>]();
            }
            pub fn [<unset_ $name>](&mut self) {
                self._p.[<unset_ $name>]();
            }
        }
    };
}

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

bitfield! {
    struct StatusFlags(u8);
    get_c, set_c_value: 0;
    get_z, set_z_value: 1;
    get_i, set_i_value: 2;
    get_d, set_d_value: 3;
    get_b, set_b_value: 4;
    get_v, set_v_value: 6;
    get_n, set_n_value: 7;
}

bitfield! {
    struct IrqRstControl(u16);
    get_nmig, set_nmig_value: 0;
    get_nmil, set_nmil_value: 1;
    get_nmip, set_nmip_value: 2;
    get_irqp, set_irqp_value: 3;
    get_intg, set_intg_value: 4;
    get_resp, set_resp_value: 5;
    get_resg, set_resg_value: 6;
    get_last_rst, set_last_rst_value: 7;
    get_last_nmig, set_last_nmig_value: 8;
    get_last_nmil, set_last_nmil_value: 9;
    get_last_irq, set_last_irq_value: 10;
    get_brk_done, set_brk_done_value: 11;
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
    _p: StatusFlags,
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
