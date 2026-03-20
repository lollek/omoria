use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwordSubType {
    Backsword,
    BastardSword,
    Broadsword,
    Claymore,
    Cutlass,
    Espadon,
    ExecutionersSword,
    Flamberge,
    Katana,
    Longsword,
    Nodachi,
    Sabre,
    Zweihander,
    BrokenSword,
}

impl From<SwordSubType> for usize {
    fn from(value: SwordSubType) -> usize {
        match value {
            SwordSubType::Backsword => 6,
            SwordSubType::BastardSword => 7,
            SwordSubType::Broadsword => 10,
            SwordSubType::Claymore => 11,
            SwordSubType::Cutlass => 12,
            SwordSubType::Espadon => 13,
            SwordSubType::ExecutionersSword => 14,
            SwordSubType::Flamberge => 15,
            SwordSubType::Katana => 17,
            SwordSubType::Longsword => 18,
            SwordSubType::Nodachi => 19,
            SwordSubType::Sabre => 21,
            SwordSubType::Zweihander => 23,
            SwordSubType::BrokenSword => 24,
        }
    }
}

impl TryFrom<usize> for SwordSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(SwordSubType::Backsword),
            7 => Ok(SwordSubType::BastardSword),
            10 => Ok(SwordSubType::Broadsword),
            11 => Ok(SwordSubType::Claymore),
            12 => Ok(SwordSubType::Cutlass),
            13 => Ok(SwordSubType::Espadon),
            14 => Ok(SwordSubType::ExecutionersSword),
            15 => Ok(SwordSubType::Flamberge),
            17 => Ok(SwordSubType::Katana),
            18 => Ok(SwordSubType::Longsword),
            19 => Ok(SwordSubType::Nodachi),
            21 => Ok(SwordSubType::Sabre),
            23 => Ok(SwordSubType::Zweihander),
            24 => Ok(SwordSubType::BrokenSword),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::SwordSubType;

    #[test]
    fn test_sword_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            SwordSubType::try_from(6usize).unwrap(),
            SwordSubType::Backsword
        );
    }

    #[test]
    fn test_sword_subtype_try_from_usize_rejects_unknown_value() {
        assert!(SwordSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_sword_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(SwordSubType::BrokenSword), 24);
    }
}
