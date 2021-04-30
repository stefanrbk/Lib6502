use super::*;

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

impl CpuIO {
    pub fn new() -> CpuIO {
        CpuIO {
            db: new_bus!(0 as u8),
            abh: new_bus!(0 as u8),
            abl: new_bus!(0 as u8),
            rw: new_pin_set!(),
            irq: new_pin_set!(),
            rdy: new_pin_unset!(),
            nmi: new_pin_set!(),
            rst: new_pin_unset!(),
            sync: new_pin_unset!(),
            so: new_pin_unset!(),
            phase_1_negative_edge: null_barrier!(),
            phase_1_positive_edge: null_barrier!(),
            phase_2_negative_edge: null_barrier!(),
            phase_2_positive_edge: null_barrier!(),
            read_write_negative_edge: null_barrier!(),
            read_write_positive_edge: null_barrier!(),
            sync_negative_edge: null_barrier!(),
            sync_positive_edge: null_barrier!(),
            addr_stable: null_barrier!(),
            data_stable: null_barrier!(),
        }
    }
}
