use std::convert::TryFrom;
use enum_iterator;

#[derive(Copy, Clone, Debug, PartialEq, enum_iterator::Sequence)]
pub enum Stat {
    Strength,
    Intelligence,
    Wisdom,
    Dexterity,
    Constitution,
    Charisma,
}

impl Stat {
    pub fn iter() -> impl Iterator<Item = Stat> {
        enum_iterator::all::<Stat>()
    }
}

impl From<Stat> for u8 {
    fn from(stat: Stat) -> Self {
        match stat {
            Stat::Strength => 0,
            Stat::Intelligence => 1,
            Stat::Wisdom => 2,
            Stat::Dexterity => 3,
            Stat::Constitution => 4,
            Stat::Charisma => 5,
        }
    }
}

impl TryFrom<u8> for Stat {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Stat::Strength),
            1 => Ok(Stat::Intelligence),
            2 => Ok(Stat::Wisdom),
            3 => Ok(Stat::Dexterity),
            4 => Ok(Stat::Constitution),
            5 => Ok(Stat::Charisma),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;
    use super::Stat;

    #[test]
    fn test_stat_iter() {
        let stats: Vec<Stat> = Stat::iter().collect();
        assert_eq!(stats.len(), 6);
        assert_eq!(stats[0], Stat::Strength);
        assert_eq!(stats[1], Stat::Intelligence);
        assert_eq!(stats[2], Stat::Wisdom);
        assert_eq!(stats[3], Stat::Dexterity);
        assert_eq!(stats[4], Stat::Constitution);
        assert_eq!(stats[5], Stat::Charisma);
    }

    #[test]
    fn test_converts_to_u8() {
        let strength: u8 = Stat::Strength.into();
        assert_eq!(strength, 0u8);
        let intelligence: u8 = Stat::Intelligence.into();
        assert_eq!(intelligence, 1u8);
        let wisdom: u8 = Stat::Wisdom.into();
        assert_eq!(wisdom, 2u8);
        let dexterity: u8 = Stat::Dexterity.into();
        assert_eq!(dexterity, 3u8);
        let constitution: u8 = Stat::Constitution.into();
        assert_eq!(constitution, 4u8);
        let charisma: u8 = Stat::Charisma.into();
        assert_eq!(charisma, 5u8);
    }

    #[test]
    fn test_converts_from_u8() {
        assert_eq!(Stat::Strength, Stat::try_from(0u8).unwrap());
        assert_eq!(Stat::Intelligence, Stat::try_from(1u8).unwrap());
        assert_eq!(Stat::Wisdom, Stat::try_from(2u8).unwrap());
        assert_eq!(Stat::Dexterity, Stat::try_from(3u8).unwrap());
        assert_eq!(Stat::Constitution, Stat::try_from(4u8).unwrap());
        assert_eq!(Stat::Charisma, Stat::try_from(5u8).unwrap());
    }
}