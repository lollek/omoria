use types::Class;

#[derive(PartialEq)]
pub enum MagicType {
    Arcane = 0,
    Divine = 1,
    Nature = 2,
    Song = 3,
    Chakra = 4,
}

#[no_mangle]
pub extern fn class_uses_magic(class: i32, magic_type: i32) -> u8 {
    match magic_type {
        x if x == MagicType::Arcane as i32 =>
            (class == Class::Mage as i32 || class == Class::Adventurer as i32) as u8,
        x if x == MagicType::Divine as i32 =>
            (class == Class::Priest as i32 || class == Class::Paladin as i32) as u8,
        x if x == MagicType::Nature as i32 =>
            (class == Class::Druid as i32 || class == Class::Ranger as i32) as u8,
        x if x == MagicType::Song as i32 =>
            (class == Class::Bard as i32 || class == Class::Rogue as i32) as u8,
        x if x == MagicType::Chakra as i32 =>
            (class == Class::Monk as i32) as u8,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_melee_bonus(class: i32) -> i8 {
    Class::from(class as usize).melee_bonus()
}

#[no_mangle]
pub extern fn class_ranged_bonus(class: i32) -> i8 {
    Class::from(class as usize).ranged_bonus()
}
