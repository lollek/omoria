use std::convert::TryFrom;

use crate::model::Currency;

/// Converts a usize to a Currency.
/// Prefer using `Currency::try_from(value).ok()` directly in new code.
#[deprecated]
pub fn from_usize(value: usize) -> Option<Currency> {
    Currency::try_from(value).ok()
}

/// Converts a Currency to a usize.
/// Prefer using `usize::from(currency)` or `.into()` directly in new code.
#[deprecated]
pub fn to_usize(value: Currency) -> usize {
    value.into()
}
