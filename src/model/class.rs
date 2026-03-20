use std::convert::TryFrom;

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Class {
    Fighter,
    Wizard,
    Cleric,
    Rogue,
    Ranger,
    Paladin,
    Druid,
    Bard,
    Adventurer,
    Monk,
    Barbarian,
}

impl From<Class> for i32 {
    fn from(class: Class) -> i32 {
        match class {
            Class::Fighter => 0,
            Class::Wizard => 1,
            Class::Cleric => 2,
            Class::Rogue => 3,
            Class::Ranger => 4,
            Class::Paladin => 5,
            Class::Druid => 6,
            Class::Bard => 7,
            Class::Adventurer => 8,
            Class::Monk => 9,
            Class::Barbarian => 10,
        }
    }
}

impl TryFrom<i32> for Class {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Class::Fighter),
            1 => Ok(Class::Wizard),
            2 => Ok(Class::Cleric),
            3 => Ok(Class::Rogue),
            4 => Ok(Class::Ranger),
            5 => Ok(Class::Paladin),
            6 => Ok(Class::Druid),
            7 => Ok(Class::Bard),
            8 => Ok(Class::Adventurer),
            9 => Ok(Class::Monk),
            10 => Ok(Class::Barbarian),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::Class;

    #[test]
    fn test_try_from_i32_accepts_known_values() {
        assert_eq!(Class::try_from(0i32).unwrap(), Class::Fighter);
        assert_eq!(Class::try_from(1i32).unwrap(), Class::Wizard);
        assert_eq!(Class::try_from(2i32).unwrap(), Class::Cleric);
        assert_eq!(Class::try_from(3i32).unwrap(), Class::Rogue);
        assert_eq!(Class::try_from(4i32).unwrap(), Class::Ranger);
        assert_eq!(Class::try_from(5i32).unwrap(), Class::Paladin);
        assert_eq!(Class::try_from(6i32).unwrap(), Class::Druid);
        assert_eq!(Class::try_from(7i32).unwrap(), Class::Bard);
        assert_eq!(Class::try_from(8i32).unwrap(), Class::Adventurer);
        assert_eq!(Class::try_from(9i32).unwrap(), Class::Monk);
        assert_eq!(Class::try_from(10i32).unwrap(), Class::Barbarian);
    }

    #[test]
    fn test_try_from_i32_rejects_unknown_values() {
        assert!(Class::try_from(11i32).is_err());
    }

    #[test]
    fn test_into_i32_returns_expected_codes() {
        let fighter: i32 = Class::Fighter.into();
        let wizard: i32 = Class::Wizard.into();
        let cleric: i32 = Class::Cleric.into();
        let rogue: i32 = Class::Rogue.into();
        let ranger: i32 = Class::Ranger.into();
        let paladin: i32 = Class::Paladin.into();
        let druid: i32 = Class::Druid.into();
        let bard: i32 = Class::Bard.into();
        let adventurer: i32 = Class::Adventurer.into();
        let monk: i32 = Class::Monk.into();
        let barbarian: i32 = Class::Barbarian.into();

        assert_eq!(fighter, 0);
        assert_eq!(wizard, 1);
        assert_eq!(cleric, 2);
        assert_eq!(rogue, 3);
        assert_eq!(ranger, 4);
        assert_eq!(paladin, 5);
        assert_eq!(druid, 6);
        assert_eq!(bard, 7);
        assert_eq!(adventurer, 8);
        assert_eq!(monk, 9);
        assert_eq!(barbarian, 10);
    }

    #[test]
    fn test_roundtrip_usize_conversion() {
        for code in 0..=10 {
            let class = Class::try_from(code).unwrap();
            let converted_code: i32 = class.into();
            assert_eq!(converted_code, code);
        }
    }
}
