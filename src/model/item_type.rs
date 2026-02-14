use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    MiscObject,
    Chest,
    MiscUsable,
    Jewelry,
    Gem,
    Bag,
    WearableGem,

    SlingAmmo,
    Bolt,
    Arrow,
    Spike,

    LightSource,

    RangedWeapon,
    HaftedWeapon,
    PoleArm,
    Dagger,
    Sword,
    Pick,
    Maul,

    GemHelm,
    Boots,
    Gloves,
    Cloak,
    Helm,
    Shield,
    HardArmor,
    SoftArmor,
    Bracers,
    Belt,

    Amulet,

    Ring,

    Staff,

    Rod,

    Wand,
    Scroll1,
    Scroll2,

    Potion1,
    Potion2,
    FlaskOfOil,

    Food,
    JunkFood,

    Chime,
    Horn,

    MagicBook,
    PrayerBook,
    Instrument,
    SongBook,

    // Not Items, but yeah
    LodgingAtInn,
    Money, /* look in detect_item for limit */
    UnseenTrap,
    SeenTrap,
    Rubble,
    OpenDoor,
    ClosedDoor,
    UpStaircase,
    DownStaircase,
    SecretDoor,
    EntranceToStore,
    UpSteepStaircase,
    DownSteepStaircase,
    Whirlpool,
}

impl From<ItemType> for usize {
    fn from(item_type: ItemType) -> usize {
        u8::from(item_type).into()
    }
}

impl From<ItemType> for u8 {
    fn from(item_type: ItemType) -> u8 {
        match item_type {
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
}

impl TryFrom<usize> for ItemType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        Self::try_from(value as u8)
    }
}

impl TryFrom<u8> for ItemType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ItemType::MiscObject),
            2 => Ok(ItemType::Chest),
            3 => Ok(ItemType::MiscUsable),
            4 => Ok(ItemType::Jewelry),
            5 => Ok(ItemType::Gem),
            6 => Ok(ItemType::Bag),
            7 => Ok(ItemType::WearableGem),

            10 => Ok(ItemType::SlingAmmo),
            11 => Ok(ItemType::Bolt),
            12 => Ok(ItemType::Arrow),
            13 => Ok(ItemType::Spike),

            15 => Ok(ItemType::LightSource),

            20 => Ok(ItemType::RangedWeapon),
            21 => Ok(ItemType::HaftedWeapon),
            22 => Ok(ItemType::PoleArm),
            23 => Ok(ItemType::Dagger),
            24 => Ok(ItemType::Sword),
            25 => Ok(ItemType::Pick),
            26 => Ok(ItemType::Maul),

            29 => Ok(ItemType::GemHelm),
            30 => Ok(ItemType::Boots),
            31 => Ok(ItemType::Gloves),
            32 => Ok(ItemType::Cloak),
            33 => Ok(ItemType::Helm),
            34 => Ok(ItemType::Shield),
            35 => Ok(ItemType::HardArmor),
            36 => Ok(ItemType::SoftArmor),
            37 => Ok(ItemType::Bracers),
            38 => Ok(ItemType::Belt),

            40 => Ok(ItemType::Amulet),

            45 => Ok(ItemType::Ring),

            55 => Ok(ItemType::Staff),

            60 => Ok(ItemType::Rod),

            65 => Ok(ItemType::Wand),
            70 => Ok(ItemType::Scroll1),
            71 => Ok(ItemType::Scroll2),

            75 => Ok(ItemType::Potion1),
            76 => Ok(ItemType::Potion2),
            77 => Ok(ItemType::FlaskOfOil),

            80 => Ok(ItemType::Food),
            81 => Ok(ItemType::JunkFood),

            85 => Ok(ItemType::Chime),
            86 => Ok(ItemType::Horn),

            90 => Ok(ItemType::MagicBook),
            91 => Ok(ItemType::PrayerBook),
            92 => Ok(ItemType::Instrument),
            93 => Ok(ItemType::SongBook),

            95 => Ok(ItemType::LodgingAtInn),
            100 => Ok(ItemType::Money),
            101 => Ok(ItemType::UnseenTrap),
            102 => Ok(ItemType::SeenTrap),
            103 => Ok(ItemType::Rubble),
            104 => Ok(ItemType::OpenDoor),
            105 => Ok(ItemType::ClosedDoor),
            107 => Ok(ItemType::UpStaircase),
            108 => Ok(ItemType::DownStaircase),
            109 => Ok(ItemType::SecretDoor),
            110 => Ok(ItemType::EntranceToStore),
            111 => Ok(ItemType::UpSteepStaircase),
            112 => Ok(ItemType::DownSteepStaircase),
            113 => Ok(ItemType::Whirlpool),
            _ => Err(()),
        }
    }
}

