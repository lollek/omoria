use std::convert::TryFrom;

use crate::model::Race;

/// Converts a usize to a Race.
/// Prefer using `Race::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(pos: usize) -> Option<Race> {
    Race::try_from(pos).ok()
}

/// Converts a Race to a usize.
/// Prefer using `usize::from(race)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(pos: &Race) -> usize {
    (*pos).into()
}
