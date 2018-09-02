use libc;

use std::ptr;
use std::mem;

use misc;

extern "C" {
    static mut inventory_list: *mut TreasureRec;
    static mut inven_ctr: libc::c_long;
}

#[derive(Serialize, Deserialize)]
struct TreasureTypeJson {
    name: String,               // Object name
    tval: libc::uint8_t,        // Catagory number
    tchar: libc::c_long,        // Character representation
    flags2: libc::uint64_t,     // MORE Special flags
    flags: libc::uint64_t,      // Special flags
    p1: libc::int64_t,          // Misc. use variable
    cost: libc::int64_t,        // Cost of item
    subval: libc::int64_t,      // Sub-category number
    weight: libc::uint16_t,     // Weight in gp's
    number: libc::uint16_t,     // Number of intems
    tohit: libc::int16_t,       // Pluses to hit
    todam: libc::int16_t,       // Pluses to damage
    ac: libc::int16_t,          // Normal AC
    toac: libc::int16_t,        // Pluses to AC
    damage: [libc::c_char; 7],  // Damage when it hits
    level: libc::int8_t,        // Dungeon level item found
}

#[derive(Serialize, Deserialize)]
pub struct TreasureRecJson {
    data: TreasureTypeJson,
    ok: libc::uint8_t,
    insides: libc::uint16_t,
    is_in: libc::uint8_t,
}

#[repr(C)]
struct TreasureType {
    name: [libc::c_char; 70],   // Object name
    tval: libc::uint8_t,        // Catagory number
    tchar: libc::c_long,        // Character representation
    flags2: libc::uint64_t,     // MORE Special flags
    flags: libc::uint64_t,      // Special flags
    p1: libc::int64_t,          // Misc. use variable
    cost: libc::int64_t,        // Cost of item
    subval: libc::int64_t,      // Sub-category number
    weight: libc::uint16_t,     // Weight in gp's
    number: libc::uint16_t,     // Number of intems
    tohit: libc::int16_t,       // Pluses to hit
    todam: libc::int16_t,       // Pluses to damage
    ac: libc::int16_t,          // Normal AC
    toac: libc::int16_t,        // Pluses to AC
    damage: [libc::c_char; 7],  // Damage when it hits
    level: libc::int8_t,        // Dungeon level item found
}

impl TreasureType {
    pub fn name_as_string(&self) -> String {
        misc::c_i8_array_to_rust_string(self.name.to_vec())
    }
}

#[repr(C)]
struct TreasureRec {
    data: TreasureType,
    ok: libc::uint8_t,
    insides: libc::uint16_t,
    is_in: libc::uint8_t,
    next: *mut TreasureRec,
}

pub fn record() -> Vec<TreasureRecJson> {
    let mut ptr = unsafe { inventory_list };
    let mut vec = Vec::new();

    while !ptr.is_null() {
        let elem = unsafe { &*ptr };
        vec.push(TreasureRecJson {
            data: TreasureTypeJson {
                name: elem.data.name_as_string(),
                tval: elem.data.tval,
                tchar: elem.data.tchar,
                flags2: elem.data.flags2,
                flags: elem.data.flags,
                p1: elem.data.p1,
                cost: elem.data.cost,
                subval: elem.data.subval,
                weight: elem.data.weight,
                number: elem.data.number,
                tohit: elem.data.tohit,
                todam: elem.data.todam,
                ac: elem.data.ac,
                toac: elem.data.toac,
                damage: elem.data.damage,
                level: elem.data.level,
            },
            ok: elem.ok,
            insides: elem.insides,
            is_in: elem.is_in,
        });

        ptr = elem.next;
    }
    vec
}

pub fn set_record(record: Vec<TreasureRecJson>) {
    fn set_treasure_rec(dest: &mut TreasureRec, src: &TreasureRecJson) {
        let mut name_array: [i8; 70] = [0; 70];
        src.data.name.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(i, x)| name_array[i] = *x as i8);

        dest.data = TreasureType {
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
        };
        dest.ok = src.ok;
        dest.insides = src.insides;
        dest.is_in = src.is_in;
        dest.next = ptr::null_mut();
    }
    fn mallocfn() -> *mut TreasureRec {
        unsafe { libc::malloc(mem::size_of::<TreasureRec>()) as *mut TreasureRec }
    }

    if record.len() == 0 {
        unsafe {
            inventory_list = ptr::null_mut();
            inven_ctr = 0;
        }
        return;
    }
    let size = record.len();
    unsafe { inventory_list = mallocfn() };
    let mut ptr = unsafe { inventory_list };

    for (i, item) in record.iter().enumerate() {
        unsafe { set_treasure_rec(&mut *ptr, item) };
        if i != size -1 {
            unsafe {
                let next = mallocfn();
                (*ptr).next = next;
                ptr = (*ptr).next;
            }
        }
    }
    unsafe { inven_ctr = size as libc::c_long };
}
