use std::convert::TryFrom;

use crate::model::Sex;

/// Converts a char to a Sex.
/// Prefer using `Sex::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_char(ch: char) -> Option<Sex> {
    Sex::try_from(ch).ok()
}
