use enum_iterator;

#[derive(Copy, Clone, enum_iterator::Sequence)]
pub enum Currency {
    Iron,
    Copper,
    Silver,
    Gold,
    Platinum,
    Mithril,
}

impl Currency {
    pub fn iter() -> impl Iterator<Item = Currency> {
        enum_iterator::all::<Currency>()
    }
}
