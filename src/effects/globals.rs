use super::{minus_ac_with_roll, ArmorDamage};
use crate::equipment::EQUIP_MAX;
use crate::model::{InventoryItem, Item, ItemType, PlayerFlags, WornFlag1};
use crate::player::damage;

extern "C" {
    static mut equipment: [Item; EQUIP_MAX];
    static mut inven_temp: InventoryItem;
    static mut player_flags: PlayerFlags;
    static mut print_stat: libc::c_long;
    fn item_name(out: *mut [libc::c_char; 82], item: *const Item);
}

fn minus_ac() -> bool {
    let before = unsafe { equipment };
    let mut after = before;
    let outcome = minus_ac_with_roll(&mut after, WornFlag1::ResistAcid, crate::rng::randint);
    let (slot, suffix) = match outcome {
        ArmorDamage::Absent => return false,
        ArmorDamage::Ineffective { equipment_slot } => {
            unsafe { inven_temp.data = before[equipment_slot] };
            return false;
        }
        ArmorDamage::Resisted { equipment_slot } => (equipment_slot, "resists damage"),
        ArmorDamage::Damaged { equipment_slot } => (equipment_slot, "is damaged"),
    };
    unsafe { inven_temp.data = before[slot] };
    let mut name = [0; 82];
    unsafe { item_name(&mut name, &before[slot]) };
    crate::term::msg_print(format!(
        "Your {} {}!",
        crate::misc::c_string_lossy(&name),
        suffix
    ));
    if matches!(outcome, ArmorDamage::Damaged { .. }) {
        unsafe {
            equipment[slot].toac = after[slot].toac;
            damage::recalculate_bonuses();
        }
    }
    true
}

pub(super) fn corrode_gas(cause: *const libc::c_char) {
    if !minus_ac() {
        let hp_damage = crate::rng::randint(8);
        crate::debug::info(format!("Damage (Corrosion Gas): {}", hp_damage));
        unsafe { damage::take_hit(hp_damage, cause) };
    }
    unsafe {
        print_stat = 1;
        if damage::damage_inventory(
            &[
                ItemType::Dagger,
                ItemType::Sword,
                ItemType::GemHelm,
                ItemType::Helm,
                ItemType::Shield,
                ItemType::HardArmor,
                ItemType::Wand,
            ],
            5,
        ) > 0
        {
            crate::term::msg_print("There is an acrid smell coming from your pack.");
            damage::redraw_stats();
        }
    }
}

pub(super) fn acid_dam(initial_damage: libc::c_long, cause: *const libc::c_char) {
    let armor_absorbed = minus_ac();
    let resistant = unsafe { player_flags.acid_resist != 0 };
    let divisor = 1 + i64::from(armor_absorbed) + 2 * i64::from(resistant);
    let hp_damage = initial_damage / divisor;
    unsafe {
        damage::take_hit(hp_damage, cause);
        print_stat = 1;
    }
    crate::debug::info(format!(
        "Damage (Acid): {} - {} = {}",
        initial_damage,
        initial_damage - hp_damage,
        hp_damage
    ));
    unsafe {
        if damage::damage_inventory(
            &[
                ItemType::MiscObject,
                ItemType::Chest,
                ItemType::Bolt,
                ItemType::Arrow,
                ItemType::RangedWeapon,
                ItemType::HaftedWeapon,
                ItemType::PoleArm,
                ItemType::Boots,
                ItemType::Gloves,
                ItemType::Cloak,
                ItemType::SoftArmor,
            ],
            3,
        ) > 0
        {
            crate::term::msg_print("There is an acrid smell coming from your pack!");
            damage::redraw_stats();
        }
    }
}
