#![crate_type = "staticlib"]

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
