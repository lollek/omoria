use libc;
use thirdparty::serde::{ BigArray, NullPtr };

use types::{ GameTime, Item, Monster };

pub const STORE_INVEN_MAX: usize = 24;
pub const MAX_STORES: usize = 12;
pub const MAX_OBJECTS: usize = 473;
pub const MAX_HEIGHT: usize = 66;
pub const MAX_WIDTH: usize = 198;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InventoryItem { //treas_rec
    pub data: Item,                 // Real item
    pub ok: libc::uint8_t,          // Transient for sorting usable items
    pub insides: libc::uint16_t,    // Something with bags?
    pub is_in: libc::uint8_t,       // Something with bags?
    #[serde(with = "NullPtr")]
    pub next: *mut InventoryItem,   // Linked list next
}

#[derive(Serialize, Deserialize)]
pub struct TreasureAndCoordinate {
    pub treasure: Item,
    pub y: libc::c_long,
    pub x: libc::c_long,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InvenRecord {
    pub scost: libc::int64_t,
    pub sitem: Item,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Store {
    pub store_open: GameTime,
    pub owner: libc::uint8_t,
    pub insult_cur: libc::int8_t,
    pub store_ctr: libc::uint8_t,
    pub store_inven: [InvenRecord; STORE_INVEN_MAX + 1],
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Cave {
    #[serde(skip_serializing, default)]
    pub cptr: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub tptr: libc::uint8_t,
    pub fval: libc::uint8_t,
    pub fopen: libc::uint8_t,
    pub fm: libc::uint8_t,
    pub pl: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub tl: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub moved: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub oct: libc::uint8_t,
    #[serde(skip_serializing, default)]
    pub h2o: libc::uint8_t,
}

#[derive(Serialize, Deserialize)]
pub struct TownRecord {
    pub town_seed: libc::c_ulong,
    pub bank: [libc::int64_t; 7],
    pub stores: [Store; MAX_STORES + 1],
}

#[derive(Serialize, Deserialize)]
pub struct DungeonRecord {
    pub cur_height: libc::c_long,
    pub cur_width: libc::c_long,
    pub max_panel_rows: libc::c_long,
    pub max_panel_cols: libc::c_long,
    pub cave: Vec<Cave>,
    pub treasure: Vec<TreasureAndCoordinate>,
    pub dun_level: libc::c_long,
    pub mon_tot_mult: libc::c_long,
    pub turn: libc::c_long,
    pub randes_seed: libc::c_long,
}

#[derive(Serialize, Deserialize)]
pub struct IdentifiedRecord {
    #[serde(with = "BigArray")]
    pub list: [libc::uint8_t; MAX_OBJECTS + 1],
}

#[derive(Serialize, Deserialize)]
pub struct MonsterRecord {
    pub monsters: Vec<Monster>,
}

