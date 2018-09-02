use std::ptr;

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

impl From<TreasureRec> for TreasureRecJson {
    fn from(other: TreasureRec) -> Self {
        let name = misc::c_i8_array_to_rust_string(other.data.name.to_vec());

        TreasureRecJson {
            data: TreasureTypeJson {
                name: name,
                tval: other.data.tval,
                tchar: other.data.tchar,
                flags2: other.data.flags2,
                flags: other.data.flags,
                p1: other.data.p1,
                cost: other.data.cost,
                subval: other.data.subval,
                weight: other.data.weight,
                number: other.data.number,
                tohit: other.data.tohit,
                todam: other.data.todam,
                ac: other.data.ac,
                toac: other.data.toac,
                damage: other.data.damage,
                level: other.data.level,
            },
            ok: other.ok,
            insides: other.insides,
            is_in: other.is_in,
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TreasureRec {
    pub data: TreasureType,
    pub ok: libc::uint8_t,
    pub insides: libc::uint16_t,
    pub is_in: libc::uint8_t,
    pub next: *mut TreasureRec,
}

impl From<TreasureRecJson> for TreasureRec {
    fn from(src: TreasureRecJson) -> Self {
        let mut name_array: [i8; 70] = [0; 70];
        src.data.name.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, x)| name_array[i] = *x as i8);

        TreasureRec {
            data: TreasureType {
                name: name_array,
                tval: src.data.tval,
                tchar: src.data.tchar,
                flags2: src.data.flags2,
                flags: src.data.flags,
                p1: src.data.p1,
                cost: src.data.cost,
                subval: src.data.subval,
                weight: src.data.weight,
                number: src.data.number,
                tohit: src.data.tohit,
                todam: src.data.todam,
                ac: src.data.ac,
                toac: src.data.toac,
                damage: src.data.damage,
                level: src.data.level,
            },
            ok: src.ok,
            insides: src.insides,
            is_in: src.is_in,
            next: ptr::null_mut(),
        }
    }
}
