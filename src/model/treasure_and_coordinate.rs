use libc;

use crate::model::Item;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct TreasureAndCoordinate {
    pub treasure: Item,
    pub y: libc::c_long,
    pub x: libc::c_long,
}
