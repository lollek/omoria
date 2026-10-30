use pancurses;

use crate::model::ItemType;
use crate::user_interface::map_colors::MapColor;

// Has damage numbers
pub fn has_damage(item_type: &ItemType) -> bool {
    matches!(
        item_type,
        ItemType::SlingAmmo
            | ItemType::Bolt
            | ItemType::Arrow
            | ItemType::Spike
            | ItemType::HaftedWeapon
            | ItemType::PoleArm
            | ItemType::Dagger
            | ItemType::Sword
            | ItemType::Pick
            | ItemType::Maul
    )
}

// Can be enhanced to-hit
pub fn has_attack_enhancement(item_type: &ItemType) -> bool {
    matches!(
        item_type,
        ItemType::SlingAmmo
            | ItemType::Bolt
            | ItemType::Arrow
            | ItemType::Spike
            | ItemType::RangedWeapon
            | ItemType::HaftedWeapon
            | ItemType::PoleArm
            | ItemType::Dagger
            | ItemType::Sword
            | ItemType::Pick
            | ItemType::Maul
    )
}

pub fn color(item_type: &ItemType, _subval: i64) -> MapColor {
    match item_type {
        ItemType::Jewelry
        | ItemType::Gem
        | ItemType::WearableGem
        | ItemType::Amulet
        | ItemType::Ring => MapColor::Gem,
        ItemType::SlingAmmo
        | ItemType::Bolt
        | ItemType::Arrow
        | ItemType::Spike
        | ItemType::RangedWeapon
        | ItemType::HaftedWeapon
        | ItemType::PoleArm
        | ItemType::Dagger
        | ItemType::Sword
        | ItemType::Pick
        | ItemType::Maul => MapColor::Weapon,
        ItemType::GemHelm
        | ItemType::Boots
        | ItemType::Gloves
        | ItemType::Cloak
        | ItemType::Helm
        | ItemType::Shield
        | ItemType::HardArmor
        | ItemType::SoftArmor
        | ItemType::Bracers
        | ItemType::Belt => MapColor::Armor,
        ItemType::Scroll1
        | ItemType::Scroll2
        | ItemType::MagicBook
        | ItemType::PrayerBook
        | ItemType::SongBook => MapColor::Scroll,
        ItemType::Potion1 | ItemType::Potion2 | ItemType::FlaskOfOil => MapColor::Potion,
        ItemType::Food | ItemType::JunkFood => MapColor::Food,
        ItemType::Money => MapColor::Gold,
        ItemType::SeenTrap => MapColor::Trap,
        ItemType::Rubble => MapColor::Rubble,
        ItemType::OpenDoor | ItemType::ClosedDoor | ItemType::SecretDoor => MapColor::Door,
        ItemType::UpStaircase
        | ItemType::DownStaircase
        | ItemType::UpSteepStaircase
        | ItemType::DownSteepStaircase => MapColor::Stairs,
        ItemType::EntranceToStore => MapColor::ShopEntrance,
        ItemType::MiscObject
        | ItemType::Chest
        | ItemType::MiscUsable
        | ItemType::Bag
        | ItemType::LightSource
        | ItemType::Staff
        | ItemType::Rod
        | ItemType::Wand
        | ItemType::Chime
        | ItemType::Horn
        | ItemType::Instrument
        | ItemType::LodgingAtInn
        | ItemType::UnseenTrap
        | ItemType::Whirlpool => MapColor::Item,
    }
}

pub fn glyph(item_type: &ItemType, subval: i64) -> pancurses::chtype {
    let symbol = symbol(item_type, subval);
    if matches!(item_type, ItemType::UnseenTrap | ItemType::SecretDoor) {
        symbol
    } else {
        symbol | crate::user_interface::map_colors::map_color_attr(color(item_type, subval))
    }
}

pub fn symbol(item_type: &ItemType, subval: i64) -> pancurses::chtype {
    match item_type {
        ItemType::MiscObject => {
            match subval {
                // Skeletons
                1 | 2 | 7 | 8 | 9 | 10 | 11 | 12 => 's',

                // Empty bottle
                4 => '!',

                // Broken garbage
                5 | 13 => '~',

                99 => '(',

                _ => panic!(),
            }
        }
        .into(),
        ItemType::Chest => {
            match subval {
                5 => 'z', // Dead human body
                _ => '&',
            }
        }
        .into(),
        ItemType::MiscUsable => {
            match subval {
                14 => '~',      // Statue
                15 => 's',      // Broken teeth
                16..=20 => '~', // Metal cross
                21 => '!',      // Bottle
                24 => '~',      // Holy hand grenade
                _ => panic!(),
            }
        }
        .into(),
        ItemType::Jewelry => '*'.into(),
        ItemType::Gem => '*'.into(),
        ItemType::Bag => '~'.into(),
        ItemType::WearableGem => '*'.into(),

        ItemType::SlingAmmo => '{'.into(),
        ItemType::Bolt => '{'.into(),
        ItemType::Arrow => '{'.into(),
        ItemType::Spike => '~'.into(),

        ItemType::LightSource => '~'.into(),

        ItemType::RangedWeapon => '}'.into(),
        ItemType::HaftedWeapon => '\\'.into(),
        ItemType::PoleArm => '/'.into(),
        ItemType::Dagger => {
            match subval {
                40 => '\\', // Cat-O-Nine Tails
                _ => '|',
            }
        }
        .into(),
        ItemType::Sword => '|'.into(),
        ItemType::Pick => '\\'.into(),
        ItemType::Maul => '\\'.into(),

        ItemType::GemHelm => ']'.into(),
        ItemType::Boots => ']'.into(),
        ItemType::Gloves => ']'.into(),
        ItemType::Cloak => '('.into(),
        ItemType::Helm => ']'.into(),
        ItemType::Shield => ')'.into(),
        ItemType::HardArmor => '['.into(),
        ItemType::SoftArmor => '('.into(),
        ItemType::Bracers => {
            match subval {
                7 | 264 | 271 | 272 | 273 => '*', // Fancy stuff
                _ => ']',                         // Sturdy stuff
            }
        }
        .into(),
        ItemType::Belt => {
            match subval {
                10 | 11 => '*', // Fancy stuff
                _ => '(',       // Sturdy stuff
            }
        }
        .into(),

        ItemType::Amulet => {
            match subval {
                268..=270 => '*', // Fancy stuff
                _ => '"',         // Magic stuff
            }
        }
        .into(),

        ItemType::Ring => '='.into(),

        ItemType::Staff => '_'.into(),

        ItemType::Rod => '-'.into(),

        ItemType::Wand => '-'.into(),
        ItemType::Scroll1 => '?'.into(),
        ItemType::Scroll2 => '?'.into(),

        ItemType::Potion1 => '!'.into(),
        ItemType::Potion2 => '!'.into(),
        ItemType::FlaskOfOil => '!'.into(),

        ItemType::Food => {
            match subval {
                2000 => 'R', // Rice-a-Roni
                4000 => 'J', // Jolly Green Jelly
                _ => ',',
            }
        }
        .into(),
        ItemType::JunkFood => ','.into(),

        ItemType::Chime => '%'.into(),
        ItemType::Horn => '%'.into(),

        ItemType::MagicBook => '?'.into(),
        ItemType::PrayerBook => '?'.into(),
        ItemType::Instrument => '%'.into(),
        ItemType::SongBook => '?'.into(),

        ItemType::LodgingAtInn => '.'.into(),
        ItemType::Money => '$'.into(),
        ItemType::UnseenTrap => '.'.into(),
        ItemType::SeenTrap => {
            match subval {
                1 => ' ', // Open pit
                6 => ';', // Loose rock
                _ => '^',
            }
        }
        .into(),
        ItemType::Rubble => ':'.into(),
        ItemType::OpenDoor => '\''.into(),
        ItemType::ClosedDoor => '+'.into(),
        ItemType::UpStaircase => '<'.into(),
        ItemType::DownStaircase => '>'.into(),
        ItemType::SecretDoor => '#'.into(),
        ItemType::EntranceToStore => {
            match subval {
                101 => 'G' as pancurses::chtype | pancurses::A_BOLD, // General store
                102 => 'R' as pancurses::chtype | pancurses::A_BOLD, // Armory
                103 => 'W' as pancurses::chtype | pancurses::A_BOLD, // Weapon smith
                104 => 'T' as pancurses::chtype | pancurses::A_BOLD, // Temple
                105 => 'A' as pancurses::chtype | pancurses::A_BOLD, // Alchemy Shop
                106 => 'M' as pancurses::chtype | pancurses::A_BOLD, // Magic shop
                107 => 'I' as pancurses::chtype | pancurses::A_BOLD, // Inn
                109 => 'L' as pancurses::chtype | pancurses::A_BOLD, // Library
                110 => 'U' as pancurses::chtype | pancurses::A_BOLD, // Music shop
                113 => 'J' as pancurses::chtype | pancurses::A_BOLD, // Gem store
                116 => 'D' as pancurses::chtype | pancurses::A_BOLD, // Deli
                118 => '+' as pancurses::chtype,                     // Black market
                108 => 'P' as pancurses::chtype | pancurses::A_BOLD, // Trading post
                111 => 'N' as pancurses::chtype | pancurses::A_BOLD, // Insurance shop
                112 => 'B' as pancurses::chtype | pancurses::A_BOLD, // Bank
                114 => 'X' as pancurses::chtype | pancurses::A_BOLD, // Money exchange
                115 => 'C' as pancurses::chtype | pancurses::A_BOLD, // Casino
                117 => 'Q' as pancurses::chtype | pancurses::A_BOLD, // Quest house
                _ => '+' as pancurses::chtype,
            }
        }
        ItemType::UpSteepStaircase => '<'.into(),
        ItemType::DownSteepStaircase => '>'.into(),
        ItemType::Whirlpool => panic!("whirlpool!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user_interface::map_colors::MapColor;
    use std::convert::TryFrom;

    #[test]
    fn every_item_type_has_a_map_color_role() {
        for raw_type in u8::MIN..=u8::MAX {
            if let Ok(item_type) = ItemType::try_from(raw_type) {
                let role = color(&item_type, 0);
                assert!(
                    MapColor::ALL.contains(&role),
                    "{:?} maps to an unknown map color role",
                    item_type
                );
            }
        }
    }

    #[test]
    fn fixtures_map_to_their_semantic_roles() {
        let fixtures = [
            (ItemType::OpenDoor, 0, MapColor::Door),
            (ItemType::ClosedDoor, 0, MapColor::Door),
            (ItemType::UpStaircase, 0, MapColor::Stairs),
            (ItemType::DownStaircase, 0, MapColor::Stairs),
            (ItemType::UpSteepStaircase, 0, MapColor::Stairs),
            (ItemType::DownSteepStaircase, 0, MapColor::Stairs),
            (ItemType::Rubble, 0, MapColor::Rubble),
            (ItemType::SeenTrap, 1, MapColor::Trap),
            (ItemType::SeenTrap, 6, MapColor::Trap),
            (ItemType::Money, 0, MapColor::Gold),
            (ItemType::Potion1, 0, MapColor::Potion),
            (ItemType::Scroll1, 0, MapColor::Scroll),
            (ItemType::Sword, 0, MapColor::Weapon),
            (ItemType::HardArmor, 0, MapColor::Armor),
            (ItemType::Food, 0, MapColor::Food),
        ];
        for (item_type, subval, expected) in fixtures {
            assert_eq!(color(&item_type, subval), expected, "{:?}", item_type);
        }

        for subval in 101..=118 {
            assert_eq!(
                color(&ItemType::EntranceToStore, subval),
                MapColor::ShopEntrance,
                "store subvalue {}",
                subval
            );
        }
    }

    #[test]
    fn glyph_keeps_the_symbol_character_and_existing_shop_bold_bit() {
        for (item_type, subval) in [
            (ItemType::ClosedDoor, 0),
            (ItemType::DownStaircase, 0),
            (ItemType::Potion1, 0),
            (ItemType::EntranceToStore, 101),
        ] {
            let plain = symbol(&item_type, subval);
            let colored = glyph(&item_type, subval);
            assert_eq!(colored & 0x7F, plain & 0x7F, "{item_type:?}");
            assert_eq!(colored & pancurses::A_BOLD, plain & pancurses::A_BOLD);
        }
    }
}
