use pancurses;

use crate::model::ItemType;

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
                14 => '~',                     // Statue
                15 => 's',                     // Broken teeth
                16 | 17 | 18 | 19 | 20 => '~', // Metal cross
                21 => '!',                     // Bottle
                24 => '~',                     // Holy hand grenade
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
                268 | 269 | 270 => '*', // Fancy stuff
                _ => '"',               // Magic stuff
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
