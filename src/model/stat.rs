#[derive(Copy, Clone)]
pub enum Stat {
    Strength = 0,
    Intelligence = 1,
    Wisdom = 2,
    Dexterity = 3,
    Constitution = 4,
    Charisma = 5,
}

impl Stat {
    pub fn iter() -> impl Iterator<Item=usize> {
        (Stat::Strength as usize)..(Stat::Charisma as usize + 1)
    }
}

impl From<u8> for Stat {
    fn from(stat: u8) -> Self {
        match stat {
            0 => Stat::Strength,
            1 => Stat::Intelligence,
            2 => Stat::Wisdom,
            3 => Stat::Dexterity,
            4 => Stat::Constitution,
            5 => Stat::Charisma,
            _ => panic!(),
        }
    }
}
