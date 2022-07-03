use model::Currency;

pub fn from_usize(value: usize) -> Option<Currency> {
    match value {
        1 => Some(Currency::Iron),
        2 => Some(Currency::Copper),
        3 => Some(Currency::Silver),
        4 => Some(Currency::Gold),
        5 => Some(Currency::Platinum),
        6 => Some(Currency::Mithril),
        _ => None,
    }
}

pub fn to_usize(value: Currency) -> usize {
    match value {
        Currency::Iron => 1,
        Currency::Copper => 2,
        Currency::Silver => 3,
        Currency::Gold => 4,
        Currency::Platinum => 5,
        Currency::Mithril => 6,
    }
}
