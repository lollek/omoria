use types::Stat;

#[derive(PartialEq)]
pub enum Magic {
    Arcane = 0,
    Divine = 1,
    Nature = 2,
    Song = 3,
    Chakra = 4,
}

impl Magic {
    pub fn modifier_stat(&self) -> Stat {
        match self {
            Magic::Arcane => Stat::Intelligence,
            Magic::Divine => Stat::Wisdom,
            Magic::Nature => Stat::Wisdom,
            Magic::Song => Stat::Charisma,
            Magic::Chakra => Stat::Wisdom,
        }
    }
}

impl From<i32> for Magic {
    fn from(pos: i32) -> Self {
        match pos {
            0 => Magic::Arcane,
            1 => Magic::Divine,
            2 => Magic::Nature,
            3 => Magic::Song,
            4 => Magic::Chakra,
            _ => panic!("pos out of range")
        }
    }
}
