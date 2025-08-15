use std::cmp::{max, min};
use std::convert::TryInto;

use super::item_template::{ItemQuality, ItemTemplate};
use super::template::*;
use crate::constants;
use crate::model;
use crate::model::{ItemCategory, WornFlag2};

/**
 * Returns a random item from the received list. Will panic if list is is_empty
 */
fn get_random_from_list(mut list: Vec<Box<dyn ItemTemplate>>) -> Box<dyn ItemTemplate> {
    if list.is_empty() {
        panic!("List contains 0 items!");
    }
    list.remove(rand::random::<usize>() % list.len())
}

/**
 * Generate an item level based around which dungeon level it should drop on
 */
pub fn generate_item_level_for_dungeon_level(dungeon_level: u8, tries: u8) -> u8 {
    // 1 / N times, we roll for the full treasure table
    let max_item_level = if rand::random::<u8>() % 30 == 0 {
        u8::MAX
    } else {
        dungeon_level
    };

    /* Random distribution skewered towards the dungeon level
     *
     * It might be preferable to have something which looks similar to a bell curve here.
     * Since we may end up generating too much crap the current way.
     */
    let mut item_level = 0;
    for _ in 0..tries {
        let curr_item_level = rand::random::<u8>() % max_item_level;
        if curr_item_level > item_level {
            item_level = curr_item_level;
        }
    }

    item_level
}

/**
 * Generate an item suitable sold by the general store
 */
pub fn generate_item_for_general_store() -> model::Item {
    let templates_to_choose_from: Vec<Box<dyn ItemTemplate>> = vec![
        Box::new(FoodTemplate::RationOfFood),
        Box::new(FoodTemplate::HardBiscuit),
        Box::new(FoodTemplate::BeefJerky),
        Box::new(FoodTemplate::FineAle),
        Box::new(FoodTemplate::FineWine),
        Box::new(MiscUsableTemplate::IronSpike),
        Box::new(LightSourceTemplate::BrassLantern),
        Box::new(LightSourceTemplate::WoodenTorch),
        Box::new(MiscUsableTemplate::FlaskOfOil),
        Box::new(PickTemplate::Shovel),
        Box::new(PickTemplate::Pick),
        Box::new(CloakTemplate::LightCloak),
    ];
    let random_template = get_random_from_list(templates_to_choose_from);
    generate(random_template, constants::STORE_ITEM_LEVEL, ItemQuality::Normal)
}

/**
 * Generate an item suitable sold by the armor smith
 */
pub fn generate_item_for_armorsmith() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();
    templates.extend(ArmorTemplate::vec());
    templates.extend(BootsTemplate::vec());
    templates.extend(GlovesTemplate::vec());
    templates.extend(HelmTemplate::vec());
    templates.extend(ShieldTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the weapon smith
 */
pub fn generate_item_for_weaponsmith() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();
    templates.extend(AmmunitionTemplate::vec());
    templates.extend(AxeTemplate::vec());
    templates.extend(BowTemplate::vec());
    templates.extend(CrossbowTemplate::vec());
    templates.extend(DaggerTemplate::vec());
    templates.extend(MaceTemplate::vec());
    templates.extend(PolearmTemplate::vec());
    templates.extend(SlingTemplate::vec());
    templates.extend(SwordTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the alchemist
 */
pub fn generate_item_for_alchemist_store() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();
    templates.extend(PotionTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the magic store
 */
pub fn generate_item_for_magic_store() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();
    templates.extend(StaffTemplate::vec());
    templates.extend(WandTemplate::vec());
    templates.extend(ScrollTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the inn
 */
pub fn generate_item_for_inn() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();

    templates.extend(LodgingAtInnTemplate::vec());
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, ItemQuality::Normal)
}

/**
 * Generate an item suitable sold by the library
 */
pub fn generate_item_for_library() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();

    templates.extend(MagicBookTemplate::vec());
    templates.extend(SongBookTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the temple
 */
pub fn generate_item_for_temple() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();

    templates.extend(PrayerBookTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the music store
 */
pub fn generate_item_for_music_store() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();

    templates.extend(InstrumentTemplate::vec());
    templates.extend(ChimeTemplate::vec());
    templates.extend(HornTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the gem store
 */
pub fn generate_item_for_gem_store() -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();

    templates.extend(AmuletTemplate::vec());
    templates.extend(RingTemplate::vec());
    templates.extend(ValuableTemplate::vec());

    templates.retain(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL);
    let item_quality = calculate_item_quality(constants::STORE_ITEM_LEVEL, false);
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL, item_quality)
}

/**
 * Generate an item suitable sold by the all night deli
 */
pub fn generate_item_for_all_night_deli() -> model::Item {
    generate(
        get_random_from_list(JunkFoodTemplate::vec()),
        constants::STORE_ITEM_LEVEL,
        ItemQuality::Normal,
    )
}

/**
 * Generate an item for a random item in the black market.
 *
 * We want it to be possible to spawn all items with enough luck,
 * so the max should be high, but it should also not be so high that
 * all items are unusable until the adventurer is filthy rich
 */
pub fn generate_item_for_black_market() -> model::Item {
    let item_level =
        generate_item_level_for_dungeon_level(14 + ((rand::random::<u8>() % 7) * 15), 6);
    let mut item = generate_item_for_item_level(item_level);
    item.cost *= 2;
    item.has_wornflag2(WornFlag2::Blackmarket);
    item
}

/**
 * Generate an item suitably dropped at a given dungeon level
 */
pub fn generate_item_for_dungeon_level(dungeon_level: u8) -> model::Item {
    let item_level = generate_item_level_for_dungeon_level(dungeon_level, 3);
    generate_item_for_item_level(item_level)
}

pub fn generate_melee_weapon(item_level: u8, item_quality: ItemQuality) -> model::Item {
    generate_item_for_item_level_of_category(item_level, ItemCategory::Weapon, item_quality)
}

/**
 * Main armor as in not boots, belts, etc
*/
pub fn generate_main_armor(item_level: u8, item_quality: ItemQuality) -> model::Item {
    generate(get_random_from_list(ArmorTemplate::vec()), item_level, item_quality)
}

pub fn generate_boots(item_level: u8, item_quality: ItemQuality) -> model::Item {
    generate(get_random_from_list(BootsTemplate::vec()), item_level, item_quality)
}

pub fn generate_belt(item_level: u8, item_quality: ItemQuality) -> model::Item {
    generate(get_random_from_list(BeltTemplate::vec()), item_level, item_quality)
}

/**
 * Generate an item which should have a given item level
 */
pub fn generate_item_for_item_level_of_category(item_level: u8, item_category: ItemCategory, item_quality: ItemQuality) -> model::Item {
    let mut templates: Vec<Box<dyn ItemTemplate>> = Vec::new();
    match item_category {
        ItemCategory::Armor => {
            templates.extend(ArmorTemplate::vec());
            templates.extend(BootsTemplate::vec());
            templates.extend(BeltTemplate::vec());
            templates.extend(BracersTemplate::vec());
            templates.extend(CloakTemplate::vec());
            templates.extend(GlovesTemplate::vec());
            templates.extend(HelmTemplate::vec());
            templates.extend(ShieldTemplate::vec());
        }
        ItemCategory::DungeonItems => {
            templates.extend(ChestTemplate::vec());
            templates.extend(MiscTemplate::vec());
        }
        ItemCategory::Jewelry => {
            templates.extend(AmuletTemplate::vec());
            templates.extend(ValuableTemplate::vec());
            templates.extend(RingTemplate::vec());
        }
        ItemCategory::MagicItem => {
            templates.extend(ChimeTemplate::vec());
            templates.extend(HornTemplate::vec());
            templates.extend(StaffTemplate::vec());
            templates.extend(WandTemplate::vec());
        }
        ItemCategory::Potion => {
            templates.extend(PotionTemplate::vec());
        }
        ItemCategory::Scroll => {
            templates.extend(ScrollTemplate::vec());
        }
        ItemCategory::Usable => {
            templates.extend(AmmunitionTemplate::vec());
            templates.extend(BagTemplate::vec());
            templates.extend(FoodTemplate::vec());
            templates.extend(LightSourceTemplate::vec());
            templates.extend(MiscUsableTemplate::vec());
            templates.extend(PickTemplate::vec());
        }
        ItemCategory::Weapon => {
            templates.extend(AxeTemplate::vec());
            templates.extend(BowTemplate::vec());
            templates.extend(CrossbowTemplate::vec());
            templates.extend(DaggerTemplate::vec());
            templates.extend(MaceTemplate::vec());
            templates.extend(PolearmTemplate::vec());
            templates.extend(SlingTemplate::vec());
            templates.extend(SwordTemplate::vec());
        }
    }

    templates.retain(|x| x.item_level() <= item_level);
    generate(get_random_from_list(templates), item_level, item_quality)
}

/**
 * Generate an item which should have a given item level
 */
pub fn generate_item_for_item_level(item_level: u8) -> model::Item {
    let item_type = match rand::random::<u8>() % 100 {
        0..=4 => ItemCategory::Jewelry,
        5..=9 => ItemCategory::MagicItem,
        10..=19 => ItemCategory::Scroll,
        20..=29 => ItemCategory::Potion,
        30..=49 => ItemCategory::Armor,
        50..=69 => ItemCategory::Weapon,
        70..=84 => {
            ItemCategory::Usable
        }
        85..=99 => {
            ItemCategory::DungeonItems
        }
        _ => panic!("Rand out of range!"),
    };

    let item_quality = calculate_item_quality(item_level, true);
    generate_item_for_item_level_of_category(item_level, item_type, item_quality)
}

/**
 * Create an item from a given type and item level
 */
pub fn generate(template: Box<dyn ItemTemplate>, item_level: u8, item_quality: ItemQuality) -> model::Item {
    let mut item = template.create(item_quality, item_level);
    item.level = item_level.try_into().unwrap_or(i8::MAX);
    item
}

fn calculate_item_quality(item_level: u8, can_be_cursed: bool) -> ItemQuality {
    // 5% odds of being cursed
    if can_be_cursed && 5 > (rand::random::<u8>() % 100) {
        return ItemQuality::Cursed;
    }

    // 1: 5%, 2: 5%...10: 5%, 15: 5%, 16: 6%, 17: 7%
    let odds_for_high_quality = max(5, item_level.checked_sub(10).unwrap_or(0));
    if odds_for_high_quality > (rand::random::<u8>() % 100) {
        return ItemQuality::HighQuality
    }

    // 50%, 40%, 30%, 20%, 10%, 5%, 5%, 5%...
    let odds_for_low_quality = max(5, 6_u8.checked_sub(item_level).unwrap_or(0) * 10);
    if odds_for_low_quality > (rand::random::<u8>() % 100) {
        return ItemQuality::LowQuality
    }

    // [0-5]: 0%, [5-10]: 5%, [10+]: 10%
    let odds_for_magic = min(10, (item_level / 5) * 5);
    let is_magic = odds_for_magic > (rand::random::<u8>() % 100);

    // 10% of magic is unique
    let is_unique = is_magic && 10 > (rand::random::<u8>() % 100);

    if is_unique {
        ItemQuality::Special
    } else {
        ItemQuality::Magic
    }
}