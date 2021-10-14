use libc;

use thirdparty::serde::BigArray;

use constants;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct IdentifiedRecord {
    #[serde(with = "BigArray")]
    pub list: [libc::uint8_t; constants::MAX_OBJECTS + 1],
}
