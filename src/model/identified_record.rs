use libc;

use crate::thirdparty::serde::BigArray;

use crate::constants;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct IdentifiedRecord {
    #[serde(with = "BigArray")]
    pub list: [libc::uint8_t; constants::MAX_OBJECTS + 1],
}
