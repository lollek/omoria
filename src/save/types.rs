use libc;

use misc;

#[derive(Serialize, Deserialize)]
pub struct TreasureTypeJson {
    pub name: String,               // Object name
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

#[derive(Serialize, Deserialize)]
pub struct TreasureRecJson {
    pub data: TreasureTypeJson,
    pub ok: libc::uint8_t,
    pub insides: libc::uint16_t,
    pub is_in: libc::uint8_t,
}

#[repr(C)]
pub struct TreasureType {
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

impl TreasureType {
    pub fn name_as_string(&self) -> String {
        misc::c_i8_array_to_rust_string(self.name.to_vec())
    }
}

#[repr(C)]
pub struct TreasureRec {
    pub data: TreasureType,
    pub ok: libc::uint8_t,
    pub insides: libc::uint16_t,
    pub is_in: libc::uint8_t,
    pub next: *mut TreasureRec,
}

