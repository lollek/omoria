use crate::model::ItemType;

pub fn from_usize(pos: usize) -> Option<ItemType> {
    match pos {
        1 => Some(ItemType::MiscObject),
        2 => Some(ItemType::Chest),
        3 => Some(ItemType::MiscUsable),
        4 => Some(ItemType::Jewelry),
        5 => Some(ItemType::Gem),
        6 => Some(ItemType::Bag),
        7 => Some(ItemType::WearableGem),

        10 => Some(ItemType::SlingAmmo),
        11 => Some(ItemType::Bolt),
        12 => Some(ItemType::Arrow),
        13 => Some(ItemType::Spike),

        15 => Some(ItemType::LightSource),

        20 => Some(ItemType::RangedWeapon),
        21 => Some(ItemType::HaftedWeapon),
        22 => Some(ItemType::PoleArm),
        23 => Some(ItemType::Dagger),
        24 => Some(ItemType::Sword),
        25 => Some(ItemType::Pick),
        26 => Some(ItemType::Maul),

        29 => Some(ItemType::GemHelm),
        30 => Some(ItemType::Boots),
        31 => Some(ItemType::Gloves),
        32 => Some(ItemType::Cloak),
        33 => Some(ItemType::Helm),
        34 => Some(ItemType::Shield),
        35 => Some(ItemType::HardArmor),
        36 => Some(ItemType::SoftArmor),
        37 => Some(ItemType::Bracers),
        38 => Some(ItemType::Belt),

        40 => Some(ItemType::Amulet),

        45 => Some(ItemType::Ring),

        55 => Some(ItemType::Staff),

        60 => Some(ItemType::Rod),

        65 => Some(ItemType::Wand),
        70 => Some(ItemType::Scroll1),
        71 => Some(ItemType::Scroll2),

        75 => Some(ItemType::Potion1),
        76 => Some(ItemType::Potion2),
        77 => Some(ItemType::FlaskOfOil),

        80 => Some(ItemType::Food),
        81 => Some(ItemType::JunkFood),

        85 => Some(ItemType::Chime),
        86 => Some(ItemType::Horn),

        90 => Some(ItemType::MagicBook),
        91 => Some(ItemType::PrayerBook),
        92 => Some(ItemType::Instrument),
        93 => Some(ItemType::SongBook),

        95 => Some(ItemType::LodgingAtInn),
        100 => Some(ItemType::Money),
        101 => Some(ItemType::UnseenTrap),
        102 => Some(ItemType::SeenTrap),
        103 => Some(ItemType::Rubble),
        104 => Some(ItemType::OpenDoor),
        105 => Some(ItemType::ClosedDoor),
        107 => Some(ItemType::UpStaircase),
        108 => Some(ItemType::DownStaircase),
        109 => Some(ItemType::SecretDoor),
        110 => Some(ItemType::EntranceToStore),
        111 => Some(ItemType::UpSteepStaircase),
        112 => Some(ItemType::DownSteepStaircase),
        113 => Some(ItemType::Whirlpool),
        _ => None,
    }
}

pub fn to_usize(pos: ItemType) -> usize {
    match pos {
        ItemType::MiscObject => 1,
        ItemType::Chest => 2,
        ItemType::MiscUsable => 3,
        ItemType::Jewelry => 4,
        ItemType::Gem => 5,
        ItemType::Bag => 6,
        ItemType::WearableGem => 7,

        ItemType::SlingAmmo => 10,
        ItemType::Bolt => 11,
        ItemType::Arrow => 12,
        ItemType::Spike => 13,

        ItemType::LightSource => 15,

        ItemType::RangedWeapon => 20,
        ItemType::HaftedWeapon => 21,
        ItemType::PoleArm => 22,
        ItemType::Dagger => 23,
        ItemType::Sword => 24,
        ItemType::Pick => 25,
        ItemType::Maul => 26,

        ItemType::GemHelm => 29,
        ItemType::Boots => 30,
        ItemType::Gloves => 31,
        ItemType::Cloak => 32,
        ItemType::Helm => 33,
        ItemType::Shield => 34,
        ItemType::HardArmor => 35,
        ItemType::SoftArmor => 36,
        ItemType::Bracers => 37,
        ItemType::Belt => 38,

        ItemType::Amulet => 40,

        ItemType::Ring => 45,

        ItemType::Staff => 55,

        ItemType::Rod => 60,

        ItemType::Wand => 65,
        ItemType::Scroll1 => 70,
        ItemType::Scroll2 => 71,

        ItemType::Potion1 => 75,
        ItemType::Potion2 => 76,
        ItemType::FlaskOfOil => 77,

        ItemType::Food => 80,
        ItemType::JunkFood => 81,

        ItemType::Chime => 85,
        ItemType::Horn => 86,

        ItemType::MagicBook => 90,
        ItemType::PrayerBook => 91,
        ItemType::Instrument => 92,
        ItemType::SongBook => 93,

        ItemType::LodgingAtInn => 95,
        ItemType::Money => 100,
        ItemType::UnseenTrap => 101,
        ItemType::SeenTrap => 102,
        ItemType::Rubble => 103,
        ItemType::OpenDoor => 104,
        ItemType::ClosedDoor => 105,
        ItemType::UpStaircase => 107,
        ItemType::DownStaircase => 108,
        ItemType::SecretDoor => 109,
        ItemType::EntranceToStore => 110,
        ItemType::UpSteepStaircase => 111,
        ItemType::DownSteepStaircase => 112,
        ItemType::Whirlpool => 113,
    }
}
