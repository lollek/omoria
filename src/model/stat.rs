use enum_iterator;

#[derive(Copy, Clone, enum_iterator::Sequence)]
pub enum Stat {
    Strength,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
}

impl Stat {
    pub fn iter() -> impl Iterator<Item=Stat> {
        enum_iterator::all::<Stat>()
    }
}