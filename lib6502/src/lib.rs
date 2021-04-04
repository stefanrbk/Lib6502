use std::sync::{Arc, RwLock};

pub mod cpu;
pub mod memory;

pub type Action = fn();
pub type Bus<T> = RwLock<T>;
pub type Barrier = Arc<std::sync::Barrier>;

#[macro_export]
macro_rules! no_barrier {
    () => {
        std::sync::Arc::new(std::sync::Barrier::new(1))
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
    Set,
    SetRecognized,
    Complete,
}

enum ResetState {
    Clear,
    Set,
    Complete,
}

pub struct CpuIO {
    pub data_bus: Bus<u8>,
    pub addr_bus: Bus<u16>,
    pub rw: Bus<bool>,
    pub irq: Bus<bool>,
    pub rdy: Bus<bool>,
    pub nmi: Bus<bool>,
    pub rst: Bus<bool>,
    pub phase_1_positive_edge: Barrier,
    pub phase_2_positive_edge: Barrier,
    pub phase_1_negative_edge: Barrier,
    pub phase_2_negative_edge: Barrier,
    pub read_write_positive_edge: Barrier,
    pub read_write_negative_edge: Barrier,
    pub nmi_positive_edge: Barrier,
    pub nmi_negative_edge: Barrier,
    pub sync_positive_edge: Barrier,
    pub sync_negative_edge: Barrier,
}

pub struct Cpu {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    nmi_state: NmiState,
    t_state: TState,
    rst_state: ResetState,
    io: CpuIO,
}
