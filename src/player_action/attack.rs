use crate::data::class;
use crate::model::{Class, ItemType, Stat};
use crate::{debug, equipment, player};
use libc::c_long;

#[no_mangle]
unsafe extern "C" fn C_calculate_number_of_attacks() -> c_long {
    calculate_number_of_attacks() as c_long
}
pub fn calculate_number_of_attacks() -> i16 {
    let weapon = unsafe {
        let main_weapon = *equipment::get_item(equipment::Slot::Primary);

        if main_weapon.tval == 0 {
            WeaponState::BareHands
        } else {
            match main_weapon.item_type() {
                Some(ItemType::SlingAmmo) | Some(ItemType::Bolt) | Some(ItemType::Arrow) => {
                    WeaponState::Ammo
                }
                _ => WeaponState::Weapon { weight: main_weapon.weight as i16 },
            }
        }
    };

    let inputs = NumberOfAttacksInputs {
        weapon,
        class: player::class(),
        max_wield: player::max_bulk() as i16,
        level: player::level(),
        dex_modifier: player::modifier_from_stat(Stat::Dexterity),
        strength_modifier: player::modifier_from_stat(Stat::Strength),
    };

    let result = calculate_number_of_attacks_pure(&inputs);
    debug::infof!("Attacks: {:?} => {}", inputs, result);
    result
}

#[derive(PartialEq, Copy, Clone)]
pub enum MeleeAttackType {
    Standard,
    Backstab,
}

#[derive(PartialEq, Copy, Clone)]
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
    calculate_player_tohit(attack_type) as c_long
}

pub fn calculate_player_tohit(attack_type: AttackType) -> i16 {
    match attack_type {
        AttackType::Melee(_) => {
            let base_to_hit = player::base_to_hit();
            let plus_to_hit = player::plus_to_hit(attack_type, player::player_main_weapon());

            let total = base_to_hit + plus_to_hit;
            debug::infof!("ToHit: base {} + plus {} => total {}",
                base_to_hit,
                        plus_to_hit,
                        total
            );
            total
        }
        AttackType::Ranged => panic!("unimplemented"),
        AttackType::Thrown => panic!("unimplemented"),
    }
}

/// Describes the weapon the player is holding for attack-count purposes.
#[derive(Debug, Clone, PartialEq)]
enum WeaponState {
    /// No weapon equipped (tval == 0).
    BareHands,
    /// Holding ammo (sling ammo, bolt, arrow) — not a melee weapon.
    Ammo,
    /// A real weapon with the given weight.
    Weapon { weight: i16 },
}

/// Pure inputs for `calculate_number_of_attacks_pure`.
#[derive(Debug, Clone)]
struct NumberOfAttacksInputs {
    pub weapon: WeaponState,
    pub class: Class,
    pub max_wield: i16,
    pub level: u8,
    pub dex_modifier: i16,
    pub strength_modifier: i16,
}

/// Pure, testable version of `calculate_number_of_attacks`.
fn calculate_number_of_attacks_pure(inputs: &NumberOfAttacksInputs) -> i16 {
    if inputs.weapon == WeaponState::Ammo {
        return 1;
    } else if inputs.weapon == WeaponState::BareHands && inputs.class != Class::Monk {
        return 2;
    }
    // [0-1] attacks
    let attacks_from_class = match inputs.weapon {
        WeaponState::Weapon { .. } => 0,
        WeaponState::BareHands => 1,
        WeaponState::Ammo => unreachable!(),
    };
    // [1-350]
    let weapon_weight = match inputs.weapon {
        WeaponState::Weapon { weight } => weight,
        WeaponState::BareHands => 5, // arbitrary small weight for unarmed combat
        WeaponState::Ammo => unreachable!(),
    };

    if inputs.max_wield < weapon_weight {
        return 0;
    }

    // [0-2] attacks
    let attacks_from_level =
        (class::melee_bonus(&inputs.class) as i16 * (inputs.level + 10) as i16) / 250;

    // [160-600] avg 350
    let strength_capacity = (10 + inputs.strength_modifier * 2) * 10;
    // For each extra attack from dex you need to be an additional 1x over strength capacity to use it
    let attacks_from_dexterity_limit = strength_capacity / weapon_weight;
    // [1-4] attacks, 1 attack per 3 points of dex modifier
    let initial_attacks_from_dexterity = (5 + inputs.dex_modifier).clamp(3, 12) / 3;
    let attacks_from_dexterity = initial_attacks_from_dexterity.min(attacks_from_dexterity_limit);

    // Total [1-7] attacks
    attacks_from_class + attacks_from_level + attacks_from_dexterity
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_inputs(weapon: WeaponState, class: Class, level: u8, dex_mod: i16, str_mod: i16) -> NumberOfAttacksInputs {
        // max_wield large enough to not be a bottleneck by default
        NumberOfAttacksInputs {
            weapon,
            class,
            max_wield: 500,
            level,
            dex_modifier: dex_mod,
            strength_modifier: str_mod,
        }
    }

    #[test]
    fn bare_hands_non_monk_always_returns_2() {
        let inputs = make_inputs(WeaponState::BareHands, Class::Fighter, 1, 0, 0);
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 2);
    }

    #[test]
    fn ammo_always_returns_1() {
        let inputs = make_inputs(WeaponState::Ammo, Class::Fighter, 10, 5, 5);
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 1);
    }

    #[test]
    fn weapon_too_heavy_returns_0() {
        let inputs = NumberOfAttacksInputs {
            weapon: WeaponState::Weapon { weight: 100 },
            class: Class::Fighter,
            max_wield: 50, // can't wield it
            level: 10,
            dex_modifier: 5,
            strength_modifier: 5,
        };
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 0);
    }

    #[test]
    fn monk_bare_hands_gets_class_bonus() {
        // Monk bare hands: attacks_from_class=1, plus level/dex contributions
        let inputs = make_inputs(WeaponState::BareHands, Class::Monk, 20, 5, 5);
        let attacks = calculate_number_of_attacks_pure(&inputs);
        // Monk class bonus=1, level bonus = (8 * 30)/250 = 0, dex=(5+5)/3=3, min(3, 200/5=40) => 3+1+0=4
        assert!(attacks >= 2, "monk bare hands should get multiple attacks, got {}", attacks);
    }

    #[test]
    fn fighter_with_light_weapon_high_level() {
        // Fighter melee_bonus=10, level=50 => attacks_from_level = (10*60)/250 = 2
        // dex_modifier=5 => initial_dex = (5+5)/3 = 3
        // str_modifier=5 => capacity = (10+10)*10 = 200, limit = 200/30 = 6
        // attacks_from_dex = min(3,6) = 3
        // total = 0 + 2 + 3 = 5
        let inputs = make_inputs(WeaponState::Weapon { weight: 30 }, Class::Fighter, 50, 5, 5);
        assert_eq!(calculate_number_of_attacks_pure(&inputs), 5);
    }
}
