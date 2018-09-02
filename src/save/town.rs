use std::mem;

use libc;

use save::types::*;

extern "C" {
    static mut stores: [Store; MAX_STORES +1];
    static mut town_seed: libc::c_ulong;
    static mut bank: [libc::int64_t; 7];
}

pub fn record() -> TownRecord {
    TownRecord {
        town_seed: unsafe { town_seed }.to_owned(),
        bank: unsafe { bank }.to_owned(),
        stores: unsafe { stores }.to_owned(),
    }
}

pub fn set_record(record: TownRecord) {
    unsafe {
        mem::replace(&mut stores, record.stores);
        town_seed = record.town_seed;
        mem::replace(&mut bank, record.bank);
    }
}
