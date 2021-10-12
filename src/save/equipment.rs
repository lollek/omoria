use libc;
use debug;
use model::Item;

const EQUIP_MAX: usize = 15;

extern "C" {
    static mut equipment: [Item; EQUIP_MAX];
    static mut equip_ctr: libc::c_long;
}

pub fn record() -> Vec<Item> {
    (0..EQUIP_MAX)
        .map(|i| unsafe { equipment[i] }.to_owned())
        .collect()
}

pub fn set_record(record: Vec<Item>) {
    if record.len() != EQUIP_MAX {
        debug::fatal(&format!("equipment.len and EQUIP_MAX differ!: {} vs {}",
                              record.len(), EQUIP_MAX));
    }

    unsafe { equip_ctr = 0 };
    for (i, item) in record.into_iter().enumerate() {
        if item.tval != 0 {
            unsafe { equip_ctr += 1 };
        }
        unsafe { equipment[i] = item };
    }
}
