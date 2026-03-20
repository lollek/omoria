use std::convert::TryFrom;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Sex {
    Female,
    Male,
}

impl From<Sex> for char {
    fn from(value: Sex) -> char {
        match value {
            Sex::Female => 'F',
            Sex::Male => 'M',
        }
    }
}

impl TryFrom<char> for Sex {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'F' => Ok(Sex::Female),
            'M' => Ok(Sex::Male),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Sex;

    #[test]
    fn test_try_from_char_accepts_known_values() {
        assert_eq!(Sex::try_from('F').unwrap(), Sex::Female);
        assert_eq!(Sex::try_from('M').unwrap(), Sex::Male);
    }

    #[test]
    fn test_try_from_char_rejects_unknown_values() {
        assert!(Sex::try_from('X').is_err());
    }

    #[test]
    fn test_into_char_returns_expected_codes() {
        let female: char = Sex::Female.into();
        let male: char = Sex::Male.into();

        assert_eq!(female, 'F');
        assert_eq!(male, 'M');
    }
}
