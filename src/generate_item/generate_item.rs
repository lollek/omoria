use std::convert::TryInto;

use constants;
use item_template;
use model;

fn get_random_from_list(
    mut list: Vec<Box<dyn item_template::ItemTemplate>>,
) -> Box<dyn item_template::ItemTemplate> {
    if list.len() == 0 {
        panic!("List contains 0 items!");
    }
    list.remove(rand::random::<usize>() % list.len())
}

/**
 * generate_item_level_for_dungeon_level()
 *
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

    return item_level;
}

pub fn generate_item_for_general_store() -> model::Item {
    let templates_to_choose_from: Vec<Box<dyn item_template::ItemTemplate>> = vec![
        Box::new(item_template::FoodTemplate::RationOfFood),
        Box::new(item_template::FoodTemplate::HardBiscuit),
        Box::new(item_template::FoodTemplate::BeefJerky),
        Box::new(item_template::FoodTemplate::FineAle),
        Box::new(item_template::FoodTemplate::FineWine),
        Box::new(item_template::MiscUsableTemplate::IronSpike),
        Box::new(item_template::LightSourceTemplate::BrassLantern),
        Box::new(item_template::LightSourceTemplate::WoodenTorch),
        Box::new(item_template::MiscUsableTemplate::FlaskOfOil),
        Box::new(item_template::PickTemplate::Shovel),
        Box::new(item_template::PickTemplate::Pick),
        Box::new(item_template::CloakTemplate::LightCloak),
    ];
    let random_template = get_random_from_list(templates_to_choose_from);
    generate(random_template, constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_armorsmith() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();
    templates.extend(item_template::ArmorTemplate::vec());
    templates.extend(item_template::BootsTemplate::vec());
    templates.extend(item_template::GlovesTemplate::vec());
    templates.extend(item_template::HelmTemplate::vec());
    templates.extend(item_template::ShieldTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();

    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_weaponsmith() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();
    templates.extend(item_template::AmmunitionTemplate::vec());
    templates.extend(item_template::AxeTemplate::vec());
    templates.extend(item_template::BowTemplate::vec());
    templates.extend(item_template::CrossbowTemplate::vec());
    templates.extend(item_template::DaggerTemplate::vec());
    templates.extend(item_template::MaceTemplate::vec());
    templates.extend(item_template::PolearmTemplate::vec());
    templates.extend(item_template::SlingTemplate::vec());
    templates.extend(item_template::SwordTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_alchemist_store() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();
    templates.extend(item_template::PotionTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_magic_store() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();
    templates.extend(item_template::StaffTemplate::vec());
    templates.extend(item_template::WandTemplate::vec());
    templates.extend(item_template::ScrollTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_inn() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();

    templates.extend(item_template::LodgingAtInnTemplate::vec());

    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_library() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();

    templates.extend(item_template::MagicBookTemplate::vec());
    templates.extend(item_template::SongBookTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_temple() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();

    templates.extend(item_template::PrayerBookTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_music_store() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();

    templates.extend(item_template::InstrumentTemplate::vec());
    templates.extend(item_template::ChimeTemplate::vec());
    templates.extend(item_template::HornTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_gem_store() -> model::Item {
    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();

    templates.extend(item_template::AmuletTemplate::vec());
    templates.extend(item_template::RingTemplate::vec());
    templates.extend(item_template::ValuableTemplate::vec());

    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= constants::STORE_MAX_ITEM_LEVEL)
        .collect();
    generate(get_random_from_list(templates), constants::STORE_ITEM_LEVEL)
}

pub fn generate_item_for_all_night_deli() -> model::Item {
    generate(
        get_random_from_list(item_template::JunkFoodTemplate::vec()),
        constants::STORE_ITEM_LEVEL,
    )
}

/**
 * generate_item_for_black_market()
 *
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
    item.flags2 |= 0x20000000; // Set "black market" bit
    return item;
}

pub fn generate_item_for_dungeon_level(dungeon_level: u8) -> model::Item {
    let item_level = generate_item_level_for_dungeon_level(dungeon_level, 3);
    generate_item_for_item_level(item_level)
}

pub fn generate_item_for_item_level(item_level: u8) -> model::Item {
    /* TODO #37
    // 1: 5%, 2: 5%...10: 5%, 15: 5%, 16: 6%, 17: 7%
    let is_high_quality = if item_level > 15 {
        item_level - 10
    } else {
        5
    } > (rand::random::<u8>() % 100);

    // 50%, 40%, 30%, 20%, 10%, 5%, 5%, 5%...
    let is_low_quality = !is_high_quality && if item_level <= 5 {
        (6 - item_level) * 10
    } else {
        5
    } > (rand::random::<u8>() % 100);

    // [0-5]: 0%, [5-10]: 5%, [10+]: 10%
    let odds_for_magic = max(10, (item_level / 5) * 5);
    let is_magic = if item_level <= 5 {
        0
    } else if 5 < item_level && item_level <= 10 {
        5
    } else {
        10
    } > (rand::random::<u8>() % 100);

    // 10% of magic is unique
    let is_unique = is_magic && 10 > (rand::random::<u8>() % 100);

    // 5% odds of being cursed
    let is_cursed = !is_magic && 5 > (rand::random::<u8>() % 100);


    // TODO Implement magic_treasure()
    */

    enum GenTreasureType {
        Armor,        // Belt, Bracers, SoftArmor, HardArmor, Shield, Helm, Cloak, Gloves, Boots
        DungeonItems, // MiscObject, Chest
        Jewelry,      // Jewelry, Gems, Ring, Amulet
        MagicItem,    // Wand, Rod, Staff, Chime, Horn
        Potion,       // Potion1, Potion2
        Scroll,       // Scroll1, Scroll2
        Usable,       // FlaskOfOil, SlingAmmo, Bolt, Arrow, Spike, Bag, LightSource, MiscUsable
        Weapon,       // RangedWeapon, HafterWeapon, PoleArm, Dagger, Sword, Pick, Maul
    }

    let item_type = match rand::random::<u8>() % 100 {
        0 | 1 | 2 | 3 | 4 => GenTreasureType::Jewelry,
        5 | 6 | 7 | 8 | 9 => GenTreasureType::MagicItem,
        10 | 11 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 19 => GenTreasureType::Scroll,
        20 | 21 | 22 | 23 | 24 | 25 | 26 | 27 | 28 | 29 => GenTreasureType::Potion,
        30 | 31 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47
        | 48 | 49 => GenTreasureType::Armor,

        50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 65 | 66 | 67
        | 68 | 69 => GenTreasureType::Weapon,
        70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 => {
            GenTreasureType::Usable
        }
        85 | 86 | 87 | 88 | 89 | 90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 | 98 | 99 => {
            GenTreasureType::DungeonItems
        }
        _ => panic!("Rand out of range!"),
    };

    let mut templates: Vec<Box<dyn item_template::ItemTemplate>> = Vec::new();
    match item_type {
        GenTreasureType::Armor => {
            templates.extend(item_template::ArmorTemplate::vec());
            templates.extend(item_template::BootsTemplate::vec());
            templates.extend(item_template::BeltTemplate::vec());
            templates.extend(item_template::BracersTemplate::vec());
            templates.extend(item_template::CloakTemplate::vec());
            templates.extend(item_template::GlovesTemplate::vec());
            templates.extend(item_template::HelmTemplate::vec());
            templates.extend(item_template::ShieldTemplate::vec());
        }
        GenTreasureType::DungeonItems => {
            templates.extend(item_template::ChestTemplate::vec());
            templates.extend(item_template::MiscTemplate::vec());
        }
        GenTreasureType::Jewelry => {
            templates.extend(item_template::AmuletTemplate::vec());
            templates.extend(item_template::ValuableTemplate::vec());
            templates.extend(item_template::RingTemplate::vec());
        }
        GenTreasureType::MagicItem => {
            templates.extend(item_template::ChimeTemplate::vec());
            templates.extend(item_template::HornTemplate::vec());
            templates.extend(item_template::StaffTemplate::vec());
            templates.extend(item_template::WandTemplate::vec());
        }
        GenTreasureType::Potion => {
            templates.extend(item_template::PotionTemplate::vec());
        }
        GenTreasureType::Scroll => {
            templates.extend(item_template::ScrollTemplate::vec());
        }
        GenTreasureType::Usable => {
            templates.extend(item_template::AmmunitionTemplate::vec());
            templates.extend(item_template::BagTemplate::vec());
            templates.extend(item_template::FoodTemplate::vec());
            templates.extend(item_template::LightSourceTemplate::vec());
            templates.extend(item_template::MiscUsableTemplate::vec());
            templates.extend(item_template::PickTemplate::vec());
        }
        GenTreasureType::Weapon => {
            templates.extend(item_template::AxeTemplate::vec());
            templates.extend(item_template::BowTemplate::vec());
            templates.extend(item_template::CrossbowTemplate::vec());
            templates.extend(item_template::DaggerTemplate::vec());
            templates.extend(item_template::MaceTemplate::vec());
            templates.extend(item_template::PolearmTemplate::vec());
            templates.extend(item_template::SlingTemplate::vec());
            templates.extend(item_template::SwordTemplate::vec());
        }
    }
    templates = templates
        .into_iter()
        .filter(|x| x.item_level() <= item_level)
        .collect();
    generate(get_random_from_list(templates), item_level)
}

pub fn generate(template: Box<dyn item_template::ItemTemplate>, item_level: u8) -> model::Item {
    let mut item = template.create();

    item.level = item_level.try_into().unwrap_or(i8::MAX);

    return item;
}
