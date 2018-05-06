#[derive(PartialEq)]
pub enum Class {
    Warrior = 0,
    Mage = 1,
    Priest = 2,
    Rogue = 3,
    Ranger = 4,
    Paladin = 5,
    Druid = 6,
    Bard = 7,
    Adventurer = 8,
    Monk = 9,
}

#[derive(PartialEq)]
pub enum MagicType {
    Arcane = 0,
    Divine = 1,
    Nature = 2,
    Song = 3,
    Chakra = 4,
}

static WARRIOR_STRING: &'static [u8] = b"Warrior\0";
static MAGE_STRING: &'static [u8] = b"Mage\0";
static PRIEST_STRING: &'static [u8] = b"Priest\0";
static ROGUE_STRING: &'static [u8] = b"Rogue\0";
static RANGER_STRING: &'static [u8] = b"Ranger\0";
static PALADIN_STRING: &'static [u8] = b"Paladin\0";
static DRUID_STRING: &'static [u8] = b"Druid\0";
static BARD_STRING: &'static [u8] = b"Bard\0";
static ADVENTURER_STRING: &'static [u8] = b"Adventurer\0";
static MONK_STRING: &'static [u8] = b"Monk\0";

static WARRIOR_STATS: &'static [i8] = &[5, -2, -2, 2, 2, 1];
static MAGE_STATS: &'static [i8] = &[-5, 3, 0, 0, -2, 0];
static PRIEST_STATS: &'static [i8] = &[0, -3, 3, -1, 1, 2];
static ROGUE_STATS: &'static [i8] = &[2, 0, -2, 3, 1, 1];
static RANGER_STATS: &'static [i8] = &[2, 0, 1, 1, 1, 2];
static PALADIN_STATS: &'static [i8] = &[3, -3, 1, 0, 2, 2];
static DRUID_STATS: &'static [i8] = &[-1, -1, 2, 0, 0, 3];
static BARD_STATS: &'static [i8] = &[2, 1, -1, 2, 0, 3];
static ADVENTURER_STATS: &'static [i8] = &[2, 2, -1, 1, 1, 0];
static MONK_STATS: &'static [i8] = &[2, 0, 2, 3, 1, 1];

#[no_mangle]
pub extern fn class_title(class: i32) -> *const u8 {
    match class {
        x if x == Class::Warrior as i32 => WARRIOR_STRING.as_ptr(),
        x if x == Class::Mage as i32 => MAGE_STRING.as_ptr(),
        x if x == Class::Priest as i32 => PRIEST_STRING.as_ptr(),
        x if x == Class::Rogue as i32 => ROGUE_STRING.as_ptr(),
        x if x == Class::Ranger as i32 => RANGER_STRING.as_ptr(),
        x if x == Class::Paladin as i32 => PALADIN_STRING.as_ptr(),
        x if x == Class::Druid as i32 => DRUID_STRING.as_ptr(),
        x if x == Class::Bard as i32 => BARD_STRING.as_ptr(),
        x if x == Class::Adventurer as i32 => ADVENTURER_STRING.as_ptr(),
        x if x == Class::Monk as i32 => MONK_STRING.as_ptr(),
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_stats(class: i32) -> *const i8 {
    match class {
        x if x == Class::Warrior as i32 => WARRIOR_STATS.as_ptr(),
        x if x == Class::Mage as i32 => MAGE_STATS.as_ptr(),
        x if x == Class::Priest as i32 => PRIEST_STATS.as_ptr(),
        x if x == Class::Rogue as i32 => ROGUE_STATS.as_ptr(),
        x if x == Class::Ranger as i32 => RANGER_STATS.as_ptr(),
        x if x == Class::Paladin as i32 => PALADIN_STATS.as_ptr(),
        x if x == Class::Druid as i32 => DRUID_STATS.as_ptr(),
        x if x == Class::Bard as i32 => BARD_STATS.as_ptr(),
        x if x == Class::Adventurer as i32 => ADVENTURER_STATS.as_ptr(),
        x if x == Class::Monk as i32 => MONK_STATS.as_ptr(),
        _ => panic!("Unknown class received"),
    }
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
pub extern fn class_expfactor(class: i32) -> f32 {
    match class {
        x if x == Class::Warrior as i32 => 0.0,
        x if x == Class::Mage as i32 => 0.3,
        x if x == Class::Priest as i32 => 0.2,
        x if x == Class::Rogue as i32 => 0.1,
        x if x == Class::Ranger as i32 => 0.3,
        x if x == Class::Paladin as i32 => 0.35,
        x if x == Class::Druid as i32 => 0.2,
        x if x == Class::Bard as i32 => 0.3,
        x if x == Class::Adventurer as i32 => 0.4,
        x if x == Class::Monk as i32 => 0.1,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_extra_health(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 10,
        x if x == Class::Mage as i32 => 0,
        x if x == Class::Priest as i32 => 3,
        x if x == Class::Rogue as i32 => 6,
        x if x == Class::Ranger as i32 => 4,
        x if x == Class::Paladin as i32 => 6,
        x if x == Class::Druid as i32 => 3,
        x if x == Class::Bard as i32 => 4,
        x if x == Class::Adventurer as i32 => 4,
        x if x == Class::Monk as i32 => 4,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_disarm_mod(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 25,
        x if x == Class::Mage as i32 => 30,
        x if x == Class::Priest as i32 => 25,
        x if x == Class::Rogue as i32 => 45,
        x if x == Class::Ranger as i32 => 30,
        x if x == Class::Paladin as i32 => 20,
        x if x == Class::Druid as i32 => 25,
        x if x == Class::Bard as i32 => 30,
        x if x == Class::Adventurer as i32 => 30,
        x if x == Class::Monk as i32 => 45,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_search_mod(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 14,
        x if x == Class::Mage as i32 => 16,
        x if x == Class::Priest as i32 => 16,
        x if x == Class::Rogue as i32 => 32,
        x if x == Class::Ranger as i32 => 24,
        x if x == Class::Paladin as i32 => 12,
        x if x == Class::Druid as i32 => 16,
        x if x == Class::Bard as i32 => 22,
        x if x == Class::Adventurer as i32 => 24,
        x if x == Class::Monk as i32 => 24,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_stealth_mod(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 1,
        x if x == Class::Mage as i32 => 2,
        x if x == Class::Priest as i32 => 2,
        x if x == Class::Rogue as i32 => 4,
        x if x == Class::Ranger as i32 => 3,
        x if x == Class::Paladin as i32 => 1,
        x if x == Class::Druid as i32 => 1,
        x if x == Class::Bard as i32 => 2,
        x if x == Class::Adventurer as i32 => 3,
        x if x == Class::Monk as i32 => 3,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_search_freq(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 38,
        x if x == Class::Mage as i32 => 36,
        x if x == Class::Priest as i32 => 32,
        x if x == Class::Rogue as i32 => 16,
        x if x == Class::Ranger as i32 => 24,
        x if x == Class::Paladin as i32 => 38,
        x if x == Class::Druid as i32 => 32,
        x if x == Class::Bard as i32 => 28,
        x if x == Class::Adventurer as i32 => 24,
        x if x == Class::Monk as i32 => 24,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_melee_bonus(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 10,
        x if x == Class::Mage as i32 => 4,
        x if x == Class::Priest as i32 => 6,
        x if x == Class::Rogue as i32 => 6,
        x if x == Class::Ranger as i32 => 6,
        x if x == Class::Paladin as i32 => 8,
        x if x == Class::Druid as i32 => 4,
        x if x == Class::Bard as i32 => 5,
        x if x == Class::Adventurer as i32 => 6,
        x if x == Class::Monk as i32 => 8,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_ranged_bonus(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 8,
        x if x == Class::Mage as i32 => 4,
        x if x == Class::Priest as i32 => 5,
        x if x == Class::Rogue as i32 => 10,
        x if x == Class::Ranger as i32 => 10,
        x if x == Class::Paladin as i32 => 6,
        x if x == Class::Druid as i32 => 7,
        x if x == Class::Bard as i32 => 6,
        x if x == Class::Adventurer as i32 => 6,
        x if x == Class::Monk as i32 => 6,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_save_mod(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => 10,
        x if x == Class::Mage as i32 => 25,
        x if x == Class::Priest as i32 => 20,
        x if x == Class::Rogue as i32 => 15,
        x if x == Class::Ranger as i32 => 20,
        x if x == Class::Paladin as i32 => 15,
        x if x == Class::Druid as i32 => 20,
        x if x == Class::Bard as i32 => 20,
        x if x == Class::Adventurer as i32 => 20,
        x if x == Class::Monk as i32 => 25,
        _ => panic!("Unknown class received"),
    }
}

#[no_mangle]
pub extern fn class_magic_resist(class: i32) -> i8 {
    match class {
        x if x == Class::Warrior as i32 => -10,
        x if x == Class::Mage as i32 => 0,
        x if x == Class::Priest as i32 => 0,
        x if x == Class::Rogue as i32 => -5,
        x if x == Class::Ranger as i32 => -5,
        x if x == Class::Paladin as i32 => -5,
        x if x == Class::Druid as i32 => -5,
        x if x == Class::Bard as i32 => -5,
        x if x == Class::Adventurer as i32 => -5,
        x if x == Class::Monk as i32 => -5,
        _ => panic!("Unknown class received"),
    }
}
