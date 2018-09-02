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

impl From<TreasureType> for TreasureTypeJson {
    fn from(data: TreasureType) -> Self {
        let name = misc::c_i8_array_to_rust_string(data.name.to_vec());

        TreasureTypeJson {
            name: name,
            tval: data.tval,
            tchar: data.tchar,
            flags2: data.flags2,
            flags: data.flags,
            p1: data.p1,
            cost: data.cost,
            subval: data.subval,
            weight: data.weight,
            number: data.number,
            tohit: data.tohit,
            todam: data.todam,
            ac: data.ac,
            toac: data.toac,
            damage: data.damage,
            level: data.level,
        }
    }
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
        TreasureRecJson {
            data: TreasureTypeJson::from(other.data),
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

impl From<TreasureTypeJson> for TreasureType {
    fn from(data: TreasureTypeJson) -> Self {
        let mut name_array: [i8; 70] = [0; 70];
        data.name.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, x)| name_array[i] = *x as i8);

        TreasureType {
            name: name_array,
            tval: data.tval,
            tchar: data.tchar,
            flags2: data.flags2,
            flags: data.flags,
            p1: data.p1,
            cost: data.cost,
            subval: data.subval,
            weight: data.weight,
            number: data.number,
            tohit: data.tohit,
            todam: data.todam,
            ac: data.ac,
            toac: data.toac,
            damage: data.damage,
            level: data.level,
        }
    }
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
        TreasureRec {
            data: TreasureType::from(src.data),
            ok: src.ok,
            insides: src.insides,
            is_in: src.is_in,
            next: ptr::null_mut(),
        }
    }
}
