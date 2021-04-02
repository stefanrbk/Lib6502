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
    pub phase_1_high: Action,
    pub phase_2_high: Action,
    pub phase_1_low: Action,
    pub phase_2_low: Action,
    pub read_write_high: Action,
    pub read_write_low: Action,
}

impl Cpu {
    fn cycle(&mut self) {
        phase_2_low();
        phase_1_high();
        phase_1_low();
        phase_2_high();
    }
}
