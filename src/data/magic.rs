use model::{Magic, Stat};

pub fn modifier_stat(magic: &Magic) -> Stat {
    match magic {
        Magic::Arcane => Stat::Intelligence,
        Magic::Divine => Stat::Wisdom,
        Magic::Nature => Stat::Wisdom,
        Magic::Song => Stat::Charisma,
        Magic::Chakra => Stat::Wisdom,
    }
}
