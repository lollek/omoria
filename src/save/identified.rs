use debug;
use identification;

pub fn record() -> identification::Data {
    identification::save()
}

pub fn set_record(record: identification::Data) {
    debug::enter("identified::set_record");
    identification::load(record);
    debug::leave("identified::set_record");
}
