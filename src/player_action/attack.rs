use crate::data::class;
use crate::model::{Class, ItemType, Stat};
use crate::{debug, equipment, player};
use libc::c_long;

pub struct Attack {
    pub number_of_attacks: i16,
    pub tohit_bonus: i16,
}

#[no_mangle]
unsafe extern "C" fn C_calculate_number_of_attacks(tohit_bonus: *mut c_long) -> c_long {
    let attack = calculate_number_of_attacks();
    *tohit_bonus = attack.tohit_bonus as c_long;
    attack.number_of_attacks as c_long
}
pub fn calculate_number_of_attacks() -> Attack {
    let (attacks_from_class, weapon_weight) = unsafe {
        let main_weapon = *equipment::get_item(equipment::Slot::Primary);

        // Bare hands
        if main_weapon.tval == 0 {
            if player::class() == Class::Monk {
                (1, 5)
            } else {
                return Attack {
                    number_of_attacks: 2,
                    tohit_bonus: -3,
                };
            }
        } else {
            match main_weapon.item_type() {
                ItemType::SlingAmmo | ItemType::Bolt | ItemType::Arrow => {
                    return Attack {
                        number_of_attacks: 1,
                        tohit_bonus: 0,
                    }
                }
                _ => (0, main_weapon.weight as i16),
            }
        }
    };

    let max_wield = player::max_bulk() as i16;
    if max_wield < weapon_weight {
        return Attack {
            number_of_attacks: 1,
            tohit_bonus: (max_wield - weapon_weight) / 10,
        };
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
    let mut number_of_attacks = attacks_from_class + attacks_from_level + attacks_from_dexterity;
    debug::infof!(
        "Attacks: class {}, level {}, dex {} raw dex ({}|{}) => total {}",
        attacks_from_class,
        attacks_from_level,
        attacks_from_dexterity,
        initial_attacks_from_dexterity,
        attacks_from_dexterity_limit,
        number_of_attacks
    );
    Attack {
        number_of_attacks,
        tohit_bonus: 0,
    }
}
