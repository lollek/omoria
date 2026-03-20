use std::convert::TryFrom;

use enum_iterator;

#[derive(Copy, Clone, Debug, PartialEq, Eq, enum_iterator::Sequence)]
pub enum Currency {
    Iron,
    Copper,
    Silver,
    Gold,
    Platinum,
    Mithril,
}

impl Currency {
    pub fn iter() -> impl Iterator<Item = Currency> {
        enum_iterator::all::<Currency>()
    }
}

impl From<Currency> for usize {
    fn from(value: Currency) -> usize {
        match value {
            Currency::Iron => 1,
            Currency::Copper => 2,
            Currency::Silver => 3,
            Currency::Gold => 4,
            Currency::Platinum => 5,
            Currency::Mithril => 6,
        }
    }
}

impl TryFrom<usize> for Currency {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Currency::Iron),
            2 => Ok(Currency::Copper),
            3 => Ok(Currency::Silver),
            4 => Ok(Currency::Gold),
            5 => Ok(Currency::Platinum),
            6 => Ok(Currency::Mithril),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Currency;

    #[test]
    fn test_try_from_usize_accepts_known_values() {
        assert_eq!(Currency::try_from(1usize).unwrap(), Currency::Iron);
        assert_eq!(Currency::try_from(2usize).unwrap(), Currency::Copper);
        assert_eq!(Currency::try_from(3usize).unwrap(), Currency::Silver);
        assert_eq!(Currency::try_from(4usize).unwrap(), Currency::Gold);
        assert_eq!(Currency::try_from(5usize).unwrap(), Currency::Platinum);
        assert_eq!(Currency::try_from(6usize).unwrap(), Currency::Mithril);
    }

    #[test]
    fn test_try_from_usize_rejects_unknown_values() {
        assert!(Currency::try_from(0usize).is_err());
        assert!(Currency::try_from(7usize).is_err());
    }

    #[test]
    fn test_into_usize_returns_expected_codes() {
        let iron: usize = Currency::Iron.into();
        let copper: usize = Currency::Copper.into();
        let silver: usize = Currency::Silver.into();
        let gold: usize = Currency::Gold.into();
        let platinum: usize = Currency::Platinum.into();
        let mithril: usize = Currency::Mithril.into();

        assert_eq!(iron, 1);
        assert_eq!(copper, 2);
        assert_eq!(silver, 3);
        assert_eq!(gold, 4);
        assert_eq!(platinum, 5);
        assert_eq!(mithril, 6);
    }

    #[test]
    fn test_roundtrip_usize_conversion() {
        for currency in Currency::iter() {
            let code: usize = currency.into();
            assert_eq!(Currency::try_from(code).unwrap(), currency);
        }
    }
}
