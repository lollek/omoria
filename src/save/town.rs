use libc;

use crate::constants;
use crate::debug;
use crate::model::{ Store, TownRecord };

extern "C" {
    static mut stores: [Store; constants::MAX_STORES +1];
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
    debug::enter("town::set_record");
    unsafe {
        stores = record.stores;
        town_seed = record.town_seed;
        bank = record.bank;
    }
    debug::leave("town::set_record");
}
