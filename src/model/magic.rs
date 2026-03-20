use std::convert::TryFrom;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum Magic {
    Arcane,
    Divine,
    Nature,
    Song,
    Chakra,
}

impl From<Magic> for i32 {
    fn from(value: Magic) -> i32 {
        match value {
            Magic::Arcane => 0,
            Magic::Divine => 1,
            Magic::Nature => 2,
            Magic::Song => 3,
            Magic::Chakra => 4,
        }
    }
}

impl TryFrom<i32> for Magic {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Magic::Arcane),
            1 => Ok(Magic::Divine),
            2 => Ok(Magic::Nature),
            3 => Ok(Magic::Song),
            4 => Ok(Magic::Chakra),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Magic;

    #[test]
    fn test_try_from_i32_accepts_known_values() {
        assert_eq!(Magic::try_from(0i32).unwrap(), Magic::Arcane);
        assert_eq!(Magic::try_from(1i32).unwrap(), Magic::Divine);
        assert_eq!(Magic::try_from(2i32).unwrap(), Magic::Nature);
        assert_eq!(Magic::try_from(3i32).unwrap(), Magic::Song);
        assert_eq!(Magic::try_from(4i32).unwrap(), Magic::Chakra);
    }

    #[test]
    fn test_try_from_i32_rejects_unknown_values() {
        assert!(Magic::try_from(5i32).is_err());
    }

    #[test]
    fn test_into_i32_returns_expected_codes() {
        let arcane: i32 = Magic::Arcane.into();
        let divine: i32 = Magic::Divine.into();
        let nature: i32 = Magic::Nature.into();
        let song: i32 = Magic::Song.into();
        let chakra: i32 = Magic::Chakra.into();

        assert_eq!(arcane, 0);
        assert_eq!(divine, 1);
        assert_eq!(nature, 2);
        assert_eq!(song, 3);
        assert_eq!(chakra, 4);
    }
}
