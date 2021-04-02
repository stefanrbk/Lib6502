pub type Action = fn();
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
    data_bus: u8,
    addr_bus: u16,
    state: TState,
    rw: bool,
    irq: bool,
    rdy: bool,
    nmi: NmiState,
    rst: ResetState,
    pub phase_1_raising_edge: Vec<Action>,
    pub phase_2_raising_edge: Vec<Action>,
    pub phase_1_falling_edge: Vec<Action>,
    pub phase_2_falling_edge: Vec<Action>,
    pub read_write_raising_edge: Vec<Action>,
    pub read_write_falling_edge: Vec<Action>,
    pub nmi_raising_edge: Vec<Action>,
    pub nmi_falling_edge: Vec<Action>,
    pub sync_raising_edge: Vec<Action>,
    pub sync_falling_edge: Vec<Action>,
}

impl Cpu {
    fn cycle(&mut self) {
        phase_2_low();
        phase_1_high();
        phase_1_low();
        phase_2_high();
    }
}
