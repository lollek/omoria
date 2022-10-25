use libc;

use crate::constants;
use crate::debug;
use crate::model::IdentifiedRecord;

extern "C" {
    static mut object_ident: [libc::uint8_t; constants::MAX_OBJECTS + 1];
}

pub fn record() -> IdentifiedRecord {
    IdentifiedRecord {
        list: unsafe { object_ident }.to_owned(),
    }
}

pub fn set_record(record: IdentifiedRecord) {
    unsafe { object_ident = record.list };
}
