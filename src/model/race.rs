use std::convert::TryFrom;

use enum_iterator;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, enum_iterator::Sequence, Debug)]
pub enum Race {
    Human,
    HalfElf,
    Elf,
    Halfling,
    Gnome,
    Dwarf,
    HalfOrc,
    HalfTroll,
    Phraint,
    Dryad,
}

impl Race {
    pub fn iter() -> impl Iterator<Item = Race> {
        enum_iterator::all::<Race>()
    }
}

impl From<Race> for usize {
    fn from(race: Race) -> usize {
        match race {
            Race::Human => 0,
            Race::HalfElf => 1,
            Race::Elf => 2,
            Race::Halfling => 3,
            Race::Gnome => 4,
            Race::Dwarf => 5,
            Race::HalfOrc => 6,
            Race::HalfTroll => 7,
            Race::Phraint => 8,
            Race::Dryad => 9,
        }
    }
}

impl TryFrom<usize> for Race {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Race::Human),
            1 => Ok(Race::HalfElf),
            2 => Ok(Race::Elf),
            3 => Ok(Race::Halfling),
            4 => Ok(Race::Gnome),
            5 => Ok(Race::Dwarf),
            6 => Ok(Race::HalfOrc),
            7 => Ok(Race::HalfTroll),
            8 => Ok(Race::Phraint),
            9 => Ok(Race::Dryad),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize() {
        serde_json::to_string(&Race::Human).expect("Failed to serialize Race");
    }

    #[test]
    fn test_try_from_usize_accepts_known_values() {
        assert_eq!(Race::try_from(0usize).unwrap(), Race::Human);
        assert_eq!(Race::try_from(1usize).unwrap(), Race::HalfElf);
        assert_eq!(Race::try_from(2usize).unwrap(), Race::Elf);
        assert_eq!(Race::try_from(3usize).unwrap(), Race::Halfling);
        assert_eq!(Race::try_from(4usize).unwrap(), Race::Gnome);
        assert_eq!(Race::try_from(5usize).unwrap(), Race::Dwarf);
        assert_eq!(Race::try_from(6usize).unwrap(), Race::HalfOrc);
        assert_eq!(Race::try_from(7usize).unwrap(), Race::HalfTroll);
        assert_eq!(Race::try_from(8usize).unwrap(), Race::Phraint);
        assert_eq!(Race::try_from(9usize).unwrap(), Race::Dryad);
    }

    #[test]
    fn test_try_from_usize_rejects_unknown_values() {
        assert!(Race::try_from(10usize).is_err());
    }

    #[test]
    fn test_into_usize_returns_expected_codes() {
        let human: usize = Race::Human.into();
        let half_elf: usize = Race::HalfElf.into();
        let elf: usize = Race::Elf.into();
        let halfling: usize = Race::Halfling.into();
        let gnome: usize = Race::Gnome.into();
        let dwarf: usize = Race::Dwarf.into();
        let half_orc: usize = Race::HalfOrc.into();
        let half_troll: usize = Race::HalfTroll.into();
        let phraint: usize = Race::Phraint.into();
        let dryad: usize = Race::Dryad.into();

        assert_eq!(human, 0);
        assert_eq!(half_elf, 1);
        assert_eq!(elf, 2);
        assert_eq!(halfling, 3);
        assert_eq!(gnome, 4);
        assert_eq!(dwarf, 5);
        assert_eq!(half_orc, 6);
        assert_eq!(half_troll, 7);
        assert_eq!(phraint, 8);
        assert_eq!(dryad, 9);
    }

    #[test]
    fn test_roundtrip_usize_conversion() {
        for race in Race::iter() {
            let code: usize = race.into();
            assert_eq!(Race::try_from(code).unwrap(), race);
        }
    }
}
