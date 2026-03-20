use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DaggerSubType {
    MainGauche,
    Misercorde,
    Stiletto,
    Bodkin,
    BrokenDagger,
    CatONineTails,
    Bilbo,
    Baselard,
    Foil,
    Rapier,
    SmallSword,
}

impl From<DaggerSubType> for usize {
    fn from(value: DaggerSubType) -> usize {
        match value {
            DaggerSubType::MainGauche => 1,
            DaggerSubType::Misercorde => 2,
            DaggerSubType::Stiletto => 3,
            DaggerSubType::Bodkin => 4,
            DaggerSubType::BrokenDagger => 5,
            DaggerSubType::CatONineTails => 6,
            DaggerSubType::Bilbo => 8,
            DaggerSubType::Baselard => 9,
            DaggerSubType::Foil => 16,
            DaggerSubType::Rapier => 20,
            DaggerSubType::SmallSword => 22,
        }
    }
}

impl TryFrom<usize> for DaggerSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(DaggerSubType::MainGauche),
            2 => Ok(DaggerSubType::Misercorde),
            3 => Ok(DaggerSubType::Stiletto),
            4 => Ok(DaggerSubType::Bodkin),
            5 => Ok(DaggerSubType::BrokenDagger),
            6 => Ok(DaggerSubType::CatONineTails),
            8 => Ok(DaggerSubType::Bilbo),
            9 => Ok(DaggerSubType::Baselard),
            16 => Ok(DaggerSubType::Foil),
            20 => Ok(DaggerSubType::Rapier),
            22 => Ok(DaggerSubType::SmallSword),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::DaggerSubType;

    #[test]
    fn test_dagger_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            DaggerSubType::try_from(1usize).unwrap(),
            DaggerSubType::MainGauche
        );
    }

    #[test]
    fn test_dagger_subtype_try_from_usize_rejects_unknown_value() {
        assert!(DaggerSubType::try_from(0usize).is_err());
    }

    #[test]
    fn test_dagger_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(DaggerSubType::SmallSword), 22);
    }
}
