use libc;

use std::ptr;
use std::mem;

use save::types::*;

extern "C" {
    static mut inventory_list: *mut TreasureRec;
    static mut inven_ctr: libc::c_long;
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
