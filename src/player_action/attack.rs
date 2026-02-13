use crate::data::class;
use crate::data::class::calculate_tohit_bonus_for_weapon_type;
use crate::model::{Class, ItemType, Stat};
use crate::player::player_ptohit;
use crate::{debug, equipment, player};
use libc::c_long;

#[no_mangle]
unsafe extern "C" fn C_calculate_number_of_attacks() -> c_long {
    calculate_number_of_attacks() as c_long
}
pub fn calculate_number_of_attacks() -> i16 {
    let (attacks_from_class, weapon_weight) = unsafe {
        let main_weapon = *equipment::get_item(equipment::Slot::Primary);

        // Bare hands
        if main_weapon.tval == 0 {
            if player::class() == Class::Monk {
                (1, 5)
            } else {
                return 2;
            }
        } else {
            match main_weapon.item_type() {
                Some(ItemType::SlingAmmo) | Some(ItemType::Bolt) | Some(ItemType::Arrow) => {
                    return 1;
                }
                _ => (0, main_weapon.weight as i16),
            }
        }
    };

    let max_wield = player::max_bulk() as i16;
    if max_wield < weapon_weight {
        return 1;
    }

    // [0-2]
    let attacks_from_level =
        (class::melee_bonus(&player::class()) as i16 * (player::level() + 10) as i16) / 250;
    let dex_modifier = player::modifier_from_stat(Stat::Dexterity);

    // [160-600] avg 350
    let strength_capacity = (10 + player::modifier_from_stat(Stat::Strength) * 2) * 10;
    // For each extra attack from dex you need to be an additional 1x over strength capacity to use it
    let attacks_from_dexterity_limit = strength_capacity / weapon_weight;
    // [1-4], 1 attack per 3 points of dex modifier
    let initial_attacks_from_dexterity = (5 + dex_modifier).clamp(3, 12) / 3;
    let attacks_from_dexterity = initial_attacks_from_dexterity.min(attacks_from_dexterity_limit);

    // [1-7]
    let number_of_attacks = attacks_from_class + attacks_from_level + attacks_from_dexterity;
    debug::infof!(
        "Attacks: class {} + level {} + dex {} (raw {}/{}) => total {}",
        attacks_from_class,
        attacks_from_level,
        attacks_from_dexterity,
        initial_attacks_from_dexterity,
        attacks_from_dexterity_limit,
        number_of_attacks
    );
    number_of_attacks
}

#[derive(PartialEq)]
pub enum MeleeAttackType {
    Standard,
    Backstab,
}

pub enum AttackType {
    Melee(MeleeAttackType),
    Ranged,
    Thrown,
}

#[no_mangle]
unsafe extern "C" fn C_calculate_player_tohit_melee(is_backstab: u8) -> c_long {
    let attack_type = if is_backstab != 0 {
        AttackType::Melee(MeleeAttackType::Backstab)
    } else {
        AttackType::Melee(MeleeAttackType::Standard)
    };
    calculate_player_tohit2(&attack_type) as c_long
}
pub fn calculate_player_tohit2(attack_type: &AttackType) -> i16 {
    let main_weapon = unsafe { *equipment::get_item(equipment::Slot::Primary) };
    match attack_type {
        AttackType::Melee(melee_attack_type) => {
            let base_to_hit = unsafe { player::player_bth };
            let base_from_class_and_level = (player::level() as i16 * class::melee_bonus(&player::class()) as i16) / 2;

            let plus_base = unsafe { player_ptohit };
            let plus_bulk = if player::max_bulk() < main_weapon.weight {
                -(player::max_bulk() as i16 - main_weapon.weight as i16) / 10
            } else {
                0
            };
            let mut plus_class = if melee_attack_type == &MeleeAttackType::Backstab {
                player::level() as i16 / 4
            } else {
                0
            };
            plus_class +=
                calculate_tohit_bonus_for_weapon_type(&player::class(), main_weapon.item_type())
                    as i16;

            let total = calculate_player_tohit(base_to_hit + base_from_class_and_level, plus_base + plus_bulk + plus_class);
            debug::infof!("ToHit: (base {}, base level*class {}) + 3x(base {}, bulk {}, class {}) => total {}",
                base_to_hit,
                base_from_class_and_level,
                        plus_base,
                        plus_bulk,
                        plus_class,
                        total
            );
            total
        }
        AttackType::Ranged => panic!("unimplemented"),
        AttackType::Thrown => panic!("unimplemented"),
    }
}

#[no_mangle]
unsafe extern "C" fn C_calculate_player_tohit(base_to_hit: c_long, plus_to_hit: c_long) -> c_long {
    calculate_player_tohit(base_to_hit as i16, plus_to_hit as i16) as c_long
}
pub fn calculate_player_tohit(base_to_hit: i16, plus_to_hit: i16) -> i16 {
    base_to_hit + (plus_to_hit * 3)
}
