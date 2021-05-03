use super::*;

type DpAction = fn(&mut DataPathControl);

pub struct DataPathControl {
    adh: u8,
    adl: u8,
    db: u8,
    sb: u8,
    adh_bus_write: Vec<DpAction>,
    adh_bus_read: Vec<DpAction>,
    adl_bus_write: Vec<DpAction>,
    adl_bus_read: Vec<DpAction>,
    db_bus_write: Vec<DpAction>,
    db_bus_read: Vec<DpAction>,
    sb_bus_write: Vec<DpAction>,
    sb_bus_read: Vec<DpAction>,
}

impl DataPathControl {
    pub fn new() -> DataPathControl {
        DataPathControl {
            adh: 0,
            adl: 0,
            db: 0,
            sb: 0,
            adh_bus_read: Vec::new(),
            adl_bus_read: Vec::new(),
            db_bus_read: Vec::new(),
            sb_bus_read: Vec::new(),
            adh_bus_write: Vec::new(),
            adl_bus_write: Vec::new(),
            db_bus_write: Vec::new(),
            sb_bus_write: Vec::new(),
        }
    }

    fn adh_to_abh(&mut self) {
        
    }
}
