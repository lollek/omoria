use crate::constants;
use crate::model::IdentifiedRecord;

extern "C" {
    static mut object_ident: [u8; constants::MAX_OBJECTS + 1];
}

pub fn record() -> IdentifiedRecord {
    IdentifiedRecord {
        list: unsafe { object_ident }.to_owned(),
    }
}

pub fn set_record(record: IdentifiedRecord) {
    unsafe { object_ident = record.list };
}
