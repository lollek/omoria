use crate::model::PlayerFlags;
use crate::model::{ItemType, Stat};
use crate::player_action::attack::attacks_per_round::calculate_number_of_attacks_pure;
use crate::player_action::attack::attacks_per_round::NumberOfAttacksInputs;
use crate::player_action::attack::attacks_per_round::WeaponState;
use crate::player_action::attack::to_hit::calculate_player_tohit_melee_pure;
use crate::player_action::attack::to_hit::calculate_player_tohit_ranged_pure;
use crate::player_action::attack::to_hit::calculate_player_tohit_thrown_pure;
use crate::{debug, equipment, player};
use libc::c_long;

mod attacks_per_round;
mod to_hit;

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
                _ => WeaponState::Weapon {
                    weight: main_weapon.weight as i16,
                },
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum MeleeAttackType {
    Standard,
    Backstab,
}

#[derive(PartialEq, Copy, Clone, Debug)]
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

#[no_mangle]
unsafe extern "C" fn C_calculate_player_tohit_ranged() -> c_long {
    calculate_player_tohit(AttackType::Ranged) as c_long
}

#[no_mangle]
unsafe extern "C" fn C_calculate_player_tohit_thrown() -> c_long {
    calculate_player_tohit(AttackType::Thrown) as c_long
}

unsafe extern "C" {
    static mut player_flags: PlayerFlags;
}

pub fn calculate_player_tohit(attack_type: AttackType) -> i16 {
    let mut plus_to_hit: i16 = 0;
    equipment::items_iter().for_each(|item| {
        plus_to_hit += item.tohit;
    });
    let result = match attack_type {
        AttackType::Melee(melee_type) => {
            calculate_player_tohit_melee_pure(&to_hit::ToHitMeleeInputs {
                attack_type: melee_type,
                maybe_item_type: player::player_main_weapon().item_type(),
                class: player::class(),
                level: player::level(),
                dex_modifier: player::modifier_from_stat(Stat::Dexterity),
                strength_modifier: player::modifier_from_stat(Stat::Strength),
                player_flags: unsafe { player_flags },
                bonus_from_equipment: plus_to_hit,
            })
        }
        AttackType::Ranged => calculate_player_tohit_ranged_pure(&to_hit::ToHitRangedInputs {
            maybe_item_type: player::player_main_weapon().item_type(),
            class: player::class(),
            level: player::level(),
            dex_modifier: player::modifier_from_stat(Stat::Dexterity),
            strength_modifier: player::modifier_from_stat(Stat::Strength),
            player_flags: unsafe { player_flags },
            bonus_from_equipment: plus_to_hit,
        }),
        AttackType::Thrown => calculate_player_tohit_thrown_pure(&to_hit::ToHitRangedInputs {
            maybe_item_type: player::player_main_weapon().item_type(),
            class: player::class(),
            level: player::level(),
            dex_modifier: player::modifier_from_stat(Stat::Dexterity),
            strength_modifier: player::modifier_from_stat(Stat::Strength),
            player_flags: unsafe { player_flags },
            bonus_from_equipment: plus_to_hit,
        }),
    };
    debug::infof!("ToHit for {:?} => {}", attack_type, result);
    result
}
