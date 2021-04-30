use super::*;

use bitfield::bitfield;

bitfield! {
    pub struct DataPathControl(u128);
    pub get_, set_: 0;
}
