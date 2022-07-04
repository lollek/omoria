use crate::model::Currency;

// Returns value in iron
pub fn value(currency: &Currency) -> i64 {
    match currency {
        Currency::Iron => 1,
        Currency::Copper => 4,
        Currency::Silver => 20,
        Currency::Gold => 240,
        Currency::Platinum => 960,
        Currency::Mithril => 12480,
    }
}
