use libc;
use thirdparty::serde::{ BigArray, NullPtr };

use types::{ GameTime };

pub const STORE_INVEN_MAX: usize = 24;
pub const MAX_STORES: usize = 12;


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

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct InvenRecord {
    scost: libc::int64_t,
    sitem: TreasureType,
}

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Store {
    store_open: GameTime,
    owner: libc::uint8_t,
    insult_cur: libc::int8_t,
    store_ctr: libc::uint8_t,
    store_inven: [InvenRecord; STORE_INVEN_MAX + 1],
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct TownRecord {
    pub town_seed: libc::c_ulong,
    pub bank: [libc::int64_t; 7],
    pub stores: [Store; MAX_STORES + 1],
}
