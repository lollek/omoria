use model::Stat;

pub fn from_usize(stat: usize) -> Option<Stat> {
    match stat {
        0 => Some(Stat::Strength),
        1 => Some(Stat::Intelligence),
        2 => Some(Stat::Wisdom),
        3 => Some(Stat::Dexterity),
        4 => Some(Stat::Constitution),
        5 => Some(Stat::Charisma),
        _ => None,
    }
}
