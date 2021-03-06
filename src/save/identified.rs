use std::mem;

use libc;

use save::types::*;

extern "C" {
    static mut object_ident: [libc::uint8_t; MAX_OBJECTS + 1];
}

pub fn record() -> IdentifiedRecord {
    IdentifiedRecord {
        list: unsafe { object_ident }.to_owned(),
    }
}

pub fn set_record(record: IdentifiedRecord) {
    mem::replace(unsafe { &mut object_ident }, record.list);
}
