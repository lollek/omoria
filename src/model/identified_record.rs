use crate::thirdparty::serde::BigArray;

use crate::constants;

#[repr(C)]
#[derive(Serialize, Deserialize)]
pub struct IdentifiedRecord {
    #[serde(with = "BigArray")]
    pub list: [u8; constants::MAX_OBJECTS + 1],
}
