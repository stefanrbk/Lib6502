use std::sync::RwLock;

pub mod cpu;
pub mod memory;

pub type Action = fn();
pub type Bus<T> = RwLock<T>;

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

pub struct Cpu {
    pc: u16,
    s: u8,
    a: u8,
    x: u8,
    y: u8,
    p: u8,
    data_bus: Bus<u8>,
    addr_bus: u16,
    state: TState,
    rw: bool,
    irq: bool,
    rdy: bool,
    nmi: NmiState,
    rst: ResetState,
    pub phase_1_positive_edge: Vec<Action>,
    pub phase_2_positive_edge: Vec<Action>,
    pub phase_1_negative_edge: Vec<Action>,
    pub phase_2_negative_edge: Vec<Action>,
    pub read_write_positive_edge: Vec<Action>,
    pub read_write_negative_edge: Vec<Action>,
    pub nmi_positive_edge: Vec<Action>,
    pub nmi_negative_edge: Vec<Action>,
    pub sync_positive_edge: Vec<Action>,
    pub sync_negative_edge: Vec<Action>,
}

pub fn thing() {}
