use libc;
use thirdparty::serde::{ BigArray, NullPtr };

use types::{ GameTime };

pub const STORE_INVEN_MAX: usize = 24;
pub const MAX_STORES: usize = 12;
pub const MAX_OBJECTS: usize = 473;
pub const MAX_HEIGHT: usize = 66;
pub const MAX_WIDTH: usize = 198;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TreasureType {
    #[serde(with = "BigArray")]
    pub name: [libc::c_char; 70],   // Object name
    pub tval: libc::uint8_t,        // Catagory number
    pub tchar: libc::c_long,        // Character representation
    pub flags2: libc::uint64_t,     // MORE Special flags
    pub flags: libc::uint64_t,      // Special flags
    pub p1: libc::int64_t,          // Misc. use variable
    pub cost: libc::int64_t,        // Cost of item
    pub subval: libc::int64_t,      // Sub-category number
    pub weight: libc::uint16_t,     // Weight in gp's
    pub number: libc::uint16_t,     // Number of intems
    pub tohit: libc::int16_t,       // Pluses to hit
    pub todam: libc::int16_t,       // Pluses to damage
    pub ac: libc::int16_t,          // Normal AC
    pub toac: libc::int16_t,        // Pluses to AC
    pub damage: [libc::c_char; 7],  // Damage when it hits
    pub level: libc::int8_t,        // Dungeon level item found
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TreasureRec {
    pub data: TreasureType,         // Real item
    pub ok: libc::uint8_t,          // ??
    pub insides: libc::uint16_t,    // Something with bags?
    pub is_in: libc::uint8_t,       // Something with bags?
    #[serde(with = "NullPtr")]
    pub next: *mut TreasureRec,     // Linked list next
}

#[derive(Serialize, Deserialize)]
pub struct TreasureAndCoordinate {
    pub treasure: TreasureType,
    pub y: libc::c_long,
    pub x: libc::c_long,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InvenRecord {
    pub scost: libc::int64_t,
    pub sitem: TreasureType,
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

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Monster {
    pub hp: libc::int16_t,      // Hit points
    pub csleep: libc::int16_t,  // Inactive counter
    pub cdis: libc::int16_t,    // Cur dis from player
    pub mptr: libc::uint16_t,   // Pointer into creature
    #[serde(skip_serializing, default)]
    pub nptr: libc::uint16_t,   // Pointer to next block
    pub cspeed: libc::int8_t,   // Movement speed

    // Note: FY and FX constrain dungeon size to 255
    pub fy: libc::uint8_t,      // Y Pointer into map
    pub fx: libc::uint8_t,      // X Pointer into map

    pub stunned: libc::int8_t,  // Rounds stunned
    pub ml: libc::uint8_t,      // On if shown
    pub confused: libc::uint8_t,// On if confused
    pub moved: libc::uint8_t,   // On if water-moved
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

