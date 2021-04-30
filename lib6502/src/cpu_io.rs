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
