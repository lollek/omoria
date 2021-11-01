use std::cmp::max;
use std::convert::TryInto;

use model;
use template;

fn get_random_from_list<T>(list: Vec<T>) -> T {
    if list.len() == 0 {
        panic!("List contains 0 items!");
    }
    list[rand::random::<usize>() % list.len()]
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
    let templates_to_choose_from: Vec<Box<dyn template::Template>> = vec![
        Box::new(template::FoodTemplate::RationOfFood),
        Box::new(template::FoodTemplate::HardBiscuit),
        Box::new(template::FoodTemplate::BeefJerky),
        Box::new(template::FoodTemplate::FineAle),
        Box::new(template::FoodTemplate::FineWine),
        Box::new(template::MiscUsableTemplate::IronSpike),
        Box::new(template::LightSourceTemplate::BrassLantern),
        Box::new(template::LightSourceTemplate::WoodenTorch),
        Box::new(template::MiscUsableTemplate::FlaskOfOil),
        Box::new(template::PickTemplate::Shovel),
        Box::new(template::PickTemplate::Pick),
        Box::new(template::CloakTemplate::LightCloak),
    ];
    let random_template = get_random_from_list(templates_to_choose_from);
    create_item(random_template, 7)
}

pub fn generate_item_for_armorsmith() -> model::Item {
    let mut templates: Vec<Box<dyn template::Template>> = Vec::new();
    let boots: Vec<Box<template::BootsTemplate>> = template::BootsTemplate::iter()
        .map(|x| Box::new(x))
        .collect();

    templates.extend(boots);

    /* TODO
    let helm: Vec<Box<dyn template::Template>> = template::HelmTemplate::iter().map(|x| Box::new(x)).collect();
    let templates: Vec<Box<dyn template::Template>> = vec![
        boots,
    ].flatten();
        template::ArmorTemplate::iter().collect(),
        template::GlovesTemplate::iter().collect(),
        template::ShieldTemplate::iter().collect(),
    .flatten()
        .iter()
        .filter(|template| template.level() <= 10)
        .collect();
        */
    create_item(get_random_from_list(templates), 7)
}

pub fn generate_item_for_weaponsmith() -> model::Item {
    /* TODO
    let daggers = template::DaggerTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let swords = template::SwordTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let axes = template::AxeTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let maces = template::MaceTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let polearms = template::PolearmTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let bows = template::BowTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let crossbows = template::CrossbowTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let slings = template::SlingTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let ammo = template::AmmunitionTemplate::iter()
        .filter(|x| x.level() <= 10)
        .collect();
    let random_template = get_random_from_list(vec![
        daggers, swords, axes, maces, polearms, bows, crossbows, slings, ammo,
    ]);
    */
    let random_template = vec![];
    create_item(random_template, 7)
}

pub fn generate_item_for_alchemist_store() -> model::Item {
    /*
    let potions = template::PotionTemplate::iter()
        .filter(|x| x.level <= 10)
        .collect();
    let random_template = get_random_from_list(potions);
    */
    let random_template = vec![];
    create_item(random_template, 7)
}

pub fn generate_item_for_magic_store() -> model::Item {
    /* TODO
     * Staff, Wand, Scroll
    let staves = template::StaffTemplate::iter()
        .filter(|x| x.level <= 10)
        .collect();
    let wands = template::WandTemplate::iter()
        .filter(|x| x.level <= 10)
        .collect();
    let scrolls = template::ScrollTemplate::iter()
        .filter(|x| x.level <= 10)
        .collect();
    let random_template = get_random_from_list(vec![
        staves, wands, scrolls,
    ].flatten());
     */
    let random_template = vec![];
    create_item(random_template, 7)
}

pub fn generate_item_for_inn() -> model::Item {
    // TODO: Inn should just generate them all in the same order every time
    let random_template = get_random_from_list(template::LodgingAtInnTemplate::iter().collect());
    create_item(Box::new(random_template), 7)
}

pub fn generate_item_for_library() -> model::Item {
    let random_template = get_random_from_list(vec![
           template::MagicBookTemplate::iter().collect(),
           template::SongBookTemplate::iter().collect(),
    ]);
    create_item(random_template, 7)
}

pub fn generate_item_for_temple() -> model::Item {
    let random_template = get_random_from_list(vec![
           template::PrayerBookTemplate::iter().collect(),
    ]);
    create_item(random_template, 7)
}

pub fn generate_item_for_music_store() -> model::Item {
    /* TODO
    let random_template = get_random_from_list(vec![
       template::InstrumentTemplate::iter().collect(),
       template::ChimeTemplate::iter().collect(),
       template::HornTemplate::iter().collect(),
    ]);
    */
    let random_template = vec![];
    create_item(get_random_from_list(random_template), 7)
}

pub fn generate_item_for_gem_store() -> model::Item {
    /* TODO
    let random_template = get_random_from_list(vec![
       template::AmuletTemplate::iter().filter(|x| x.level <= 10).collect(),
       template::RingTemplate::iter().filter(|x| x.level <= 10).collect(),
       template::ValuableTemplate::iter().filter(|x| x.level <= 10).collect(),
    ]);
    */
    let random_template = vec![];
    create_item(get_random_from_list(random_template), 7)
}

pub fn generate_item_for_all_night_deli() -> model::Item {
    get_random_from_list(template::JunkFoodTemplate::iter().collect()).create()
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
    let  item_level = generate_item_level_for_dungeon_level(14 + ((rand::random::<u8>() % 7) * 15), 6);
    let mut item = generate_item_for_item_level(item_level);
    item.cost *= 2;
    item.flags2 |= 0x20000000; // Set "black market" bit
    return item
}

pub fn generate_item_for_dungeon_level(dungeon_level: u8) -> model::Item {
    let item_level = generate_item_level_for_dungeon_level(dungeon_level, 3);
    generate_item_for_item_level(item_level)
}

pub fn generate_item_for_item_level(item_level: u8) -> model::Item {

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


    // TODO: Implement magic_treasure()

    enum GenTreasureType {
        Armor, // Belt, Bracers, SoftArmor, HardArmor, Shield, Helm, Cloak, Gloves, Boots
        DungeonItems, // MiscObject, Chest
        Jewelry, // Jewelry, Gems, Ring, Amulet
        MagicItem, // Wand, Rod, Staff, Chime, Horn
        Potion, // Potion1, Potion2
        Scroll, // Scroll1, Scroll2
        Usable, // FlaskOfOil, SlingAmmo, Bolt, Arrow, Spike, Bag, LightSource, MiscUsable
        Weapon, // RangedWeapon, HafterWeapon, PoleArm, Dagger, Sword, Pick, Maul
    }

    let item_type = match rand::random::<u8>() % 100 {
        0|   1|  2|  3|  4 => GenTreasureType::Jewelry,
        5|   6|  7|  8|  9 => GenTreasureType::MagicItem,
        10| 11| 12| 13| 14|
        15| 16| 17| 18| 19 => GenTreasureType::Scroll,
        20| 21| 22| 23| 24|
        25| 26| 27| 28| 29 => GenTreasureType::Potion,
        30| 31| 32| 33| 34|
        35| 36| 37| 38| 39|
        40| 41| 42| 43| 44|
        45| 46| 47| 48| 49 => GenTreasureType::Armor,

        50| 51| 52| 53| 54|
        55| 56| 57| 58| 59|
        60| 61| 62| 63| 64|
        65| 66| 67| 68| 69 => GenTreasureType::Weapon,
        70| 71| 72| 73| 74|
        75| 76| 77| 78| 79|
        80| 81| 82| 83| 84 => GenTreasureType::Usable,
        85| 86| 87| 88| 89|
        90| 91| 92| 93| 94|
        95| 96| 97| 98| 99 => GenTreasureType::DungeonItems,
        _ => panic!("Rand out of range!"),
    };

    match item_type {
        GenTreasureType::Armor => {
            /* TODO
               Armor(template::ArmorTemplate),
               Boots(template::BootsTemplate),
               Belt(template::BeltTemplate),
               Bracers(template::BracersTemplate),
               Cloak(template::CloakTemplate),
               Gloves(template::GlovesTemplate),
               Helm(template::HelmTemplate),
               Shield(template::ShieldTemplate),
             */
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::DungeonItems => {
            /* TODO
               Chest(template::ChestTemplate),
               Misc(template::MiscTemplate),
             */
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::Jewelry => {
            /* TODO
               Amulet(template::AmuletTemplate),
               Valuable(template::ValuableTemplate),
               Ring(template::RingTemplate),
             */
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::MagicItem => {
            /* TODO
               Chime(template::ChimeTemplate),
               Horn(template::HornTemplate),
               Staff(template::StaffTemplate),
               Wand(template::WandTemplate),
             */
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::Potion => {
            // TODO
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::Scroll => {
            // TODO
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::Usable => {
            /* TODO
               Ammo(template::AmmunitionTemplate),
               Bag(template::BagTemplate),
               Food(template::FoodTemplate),
               LightSource(template::LightSourceTemplate),
               MiscUsable(template::MiscUsableTemplate),
               Pick(template::PickTemplate),
             */
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
        GenTreasureType::Weapon => {
            /* TODO
            let list = vec![
                template::AxeTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Axe(x))
                    .collect(),
                template::BowTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Bow(x))
                    .collect(),
                template::CrossbowTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Crossbow(x))
                    .collect(),
                template::DaggerTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Dagger(x))
                    .collect(),
                template::MaceTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Mace(x))
                    .collect(),
                template::PolearmTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Polearm(x))
                    .collect(),
                template::SlingTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Sling(x))
                    .collect(),
                template::SwordTemplate::iter()
                    .filter(|x| item_level >= x.level())
                    .map(|x| template::Template::Sword(x))
                    .collect(),
            ].flatten();
            */
            let list = vec![];
            let template = get_random_from_list(list);
            create_item(template, item_level)
        },
    }
}

pub fn create_item(template: Box<dyn template::Template>, item_level: u8) -> model::Item {
    let mut item = template.create();

    item.level = item_level.try_into().unwrap_or(i8::MAX);

    return item;
}

