use std::cmp::{ min, max };

use model::Item;
use template;

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

pub fn generate_item_for_dungeon_level(dungeon_level: u8) -> Item {
    let item_level = generate_item_level_for_dungeon_level(dungeon_level, 3);

    // 50%, 40%, 30%, 20%, 10%, 5%, 5%, 5%...
    let odds_for_low_quality = min(max(0, 6 - dungeon_level) * 10, 5);
    let is_low_quality = odds_for_low_quality > (rand::random::<u8>() % 100);

    // 1: 5%, 2: 5%...10: 5%, 15: 5%, 16: 6%, 17: 7%
    let odds_for_high_quality = min(max(0, dungeon_level - 10), 5);
    let is_high_quality = odds_for_high_quality > (rand::random::<u8>() % 100);

    // 5% odds of being cursed
    let odds_for_cursed = 5;
    let is_cursed = odds_for_cursed > (rand::random::<u8>() % 100);

    // [0-5]: 0%, [5-10]: 5%, [10+]: 10%
    let odds_for_magic = max(10, (dungeon_level / 5) * 5);
    let is_magic = odds_for_magic > (rand::random::<u8>() % 100);

    // 10% of magic is unique
    let odds_for_unique = 10;
    let is_unique = is_magic && 10 > (rand::random::<u8>() % 100);

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
        GenTreasureType::Armor => template::generate_armor_types(item_level),
        GenTreasureType::DungeonItems => template::generate_dungeon_items(item_level),
        GenTreasureType::Jewelry => template::generate_jewelry(item_level),
        GenTreasureType::MagicItem => template::generate_magic_item(item_level),
        GenTreasureType::Potion => template::generate_potion(item_level),
        GenTreasureType::Scroll => template::generate_scroll(item_level),
        GenTreasureType::Usable => template::generate_usable(item_level),
        GenTreasureType::Weapon => template::generate_weapon(item_level),
    }
}
