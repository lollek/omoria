use pancurses::{chtype, A_BOLD};

#[derive(Debug, PartialEq)]
pub enum ItemType {
    MiscObject = 1,
    Chest = 2,
    MiscUsable = 3,
    Jewelry = 4,
    Gem = 5,
    Bag = 6,
    WearableGem = 7,

    SlingAmmo = 10,
    Bolt = 11,
    Arrow = 12,
    Spike = 13,

    LightSource = 15,

    Bow = 16,
    Crossbow = 17,
    Sling = 18,

    Axe = 21,
    Polearm = 22,
    Dagger = 23,
    Sword = 24,
    Pick = 25,
    Mace = 26,

    Boots = 30,
    Gloves = 31,
    Cloak = 32,
    Helm = 33,
    Shield = 34,
    HardArmor = 35,
    SoftArmor = 36,
    Bracers = 37,
    Belt = 38,

    Amulet = 40,

    Ring = 45,

    Staff = 55,

    Wand = 65,
    Scroll = 70,

    Potion = 75,
    FlaskOfOil = 77,

    Food = 80,
    JunkFood = 81,

    Chime = 85,
    Horn = 86,

    MagicBook = 90,
    PrayerBook = 91,
    Instrument = 92,
    SongBook = 93,

    // Not Items, but yeah
    LodgingAtInn = 95,
    Money = 100, /* look in detect_item for limit */
    UnseenTrap = 101,
    SeenTrap = 102,
    Rubble = 103,
    OpenDoor = 104,
    ClosedDoor = 105,
    UpStaircase = 107,
    DownStaircase = 108,
    SecretDoor = 109,
    EntranceToStore = 110,
    UpSteepStaircase = 111,
    DownSteepStaircase = 112,
    Whirlpool = 113,
}

impl ItemType {
    pub fn has_damage(&self) -> bool {
        match self {
            ItemType::SlingAmmo | ItemType::Bolt | ItemType::Arrow | ItemType::Spike
                | ItemType::Axe | ItemType::Polearm
                | ItemType::Dagger | ItemType::Sword | ItemType::Pick | ItemType::Mace
                => true,
            _ => false,
        }
    }

    pub fn has_attack_enhancement(&self) -> bool {
        match self {
            ItemType::SlingAmmo | ItemType::Bolt | ItemType::Arrow | ItemType::Spike
                | ItemType::Bow | ItemType::Crossbow | ItemType::Sling | ItemType::Axe | ItemType::Polearm
                | ItemType::Dagger | ItemType::Sword | ItemType::Pick | ItemType::Mace
                => true,
            _ => false,
        }
    }

    pub fn symbol(&self, subval: i64) -> chtype {
        match self {
            ItemType::MiscObject => {
                match subval {
                    // Skeletons
                    1|2|7|8|9|10|11|12 => 's',

                    // Empty bottle
                    4 => '!',

                    // Broken garbage
                    5|13 => '~',

                    99 => '(',

                    _ => panic!(),
                }
            }.into(),
            ItemType::Chest => {
                match subval {
                    5 => 'z',   // Dead human body
                    _ => '&',
                }
            }.into(),
            ItemType::MiscUsable => {
                match subval {
                    14 => '~', // Statue
                    15 => 's', // Broken teeth
                    16|17|18|19|20 => '~', // Metal cross
                    21 => '!', // Bottle
                    24 => '~', // Holy hand grenade
                    _ => panic!(),
                }
            }.into(),
            ItemType::Jewelry => '*'.into(),
            ItemType::Gem => '*'.into(),
            ItemType::Bag => '~'.into(),
            ItemType::WearableGem => '*'.into(),

            ItemType::SlingAmmo => '{'.into(),
            ItemType::Bolt => '{'.into(),
            ItemType::Arrow => '{'.into(),
            ItemType::Spike => '~'.into(),

            ItemType::LightSource => '~'.into(),

            ItemType::Bow => '}'.into(),
            ItemType::Crossbow => '}'.into(),
            ItemType::Sling => '}'.into(),

            ItemType::Axe => '\\'.into(),
            ItemType::Polearm => '/'.into(),
            ItemType::Dagger => {
                match subval {
                    40 => '\\', // Cat-O-Nine Tails
                    _ => '|',
                }
            }.into(),
            ItemType::Sword => '|'.into(),
            ItemType::Pick => '\\'.into(),
            ItemType::Mace => '\\'.into(),

            ItemType::Boots => ']'.into(),
            ItemType::Gloves => ']'.into(),
            ItemType::Cloak => '('.into(),
            ItemType::Helm => ']'.into(),
            ItemType::Shield => ')'.into(),
            ItemType::HardArmor => '['.into(),
            ItemType::SoftArmor => '('.into(),
            ItemType::Bracers => {
                match subval {
                    7|264|271|272|273 => '*', // Fancy stuff
                    _ => ']', // Sturdy stuff
                }
            }.into(),
            ItemType::Belt => {
                match subval {
                    10|11 => '*', // Fancy stuff
                    _ => '(', // Sturdy stuff
                }
            }.into(),

            ItemType::Amulet => {
                match subval {
                    268|269|270 => '*', // Fancy stuff
                    _ => '"', // Magic stuff
                }
            }.into(),

            ItemType::Ring => '='.into(),

            ItemType::Staff => '_'.into(),

            ItemType::Wand => '-'.into(),
            ItemType::Scroll => '?'.into(),

            ItemType::Potion => '!'.into(),
            ItemType::FlaskOfOil => '!'.into(),

            ItemType::Food => {
                match subval {
                    2000 => 'R', // Rice-a-Roni
                    4000 => 'J', // Jolly Green Jelly
                    _ => ',',
                }
            }.into(),
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
            }.into(),
            ItemType::Rubble => ':'.into(),
            ItemType::OpenDoor => '\''.into(),
            ItemType::ClosedDoor => '+'.into(),
            ItemType::UpStaircase => '<'.into(),
            ItemType::DownStaircase => '>'.into(),
            ItemType::SecretDoor => '#'.into(),
            ItemType::EntranceToStore => {
                match subval {
                    101 => 'G' as chtype| A_BOLD,    // General store
                    102 => 'R' as chtype| A_BOLD,    // Armory
                    103 => 'W' as chtype| A_BOLD,    // Weapon smith
                    104 => 'T' as chtype| A_BOLD,    // Temple
                    105 => 'A' as chtype| A_BOLD,    // Alchemy Shop
                    106 => 'M' as chtype| A_BOLD,    // Magic shop
                    107 => 'I' as chtype| A_BOLD,    // Inn
                    109 => 'L' as chtype| A_BOLD,    // Library
                    110 => 'U' as chtype| A_BOLD,    // Music shop
                    113 => 'J' as chtype| A_BOLD,    // Gem store
                    116 => 'D' as chtype| A_BOLD,    // Deli
                    118 => '+' as chtype,            // Black market
                    108 => 'P' as chtype| A_BOLD,    // Trading post
                    111 => 'N' as chtype| A_BOLD,    // Insurance shop
                    112 => 'B' as chtype| A_BOLD,    // Bank
                    114 => 'X' as chtype| A_BOLD,    // Money exchange
                    115 => 'C' as chtype| A_BOLD,    // Casino
                    117 => 'Q' as chtype| A_BOLD,    // Quest house
                    _   => '+' as chtype,
                }
            },
            ItemType::UpSteepStaircase => '<'.into(),
            ItemType::DownSteepStaircase => '>'.into(),
            ItemType::Whirlpool => panic!("whirlpool!"),
        }
    }
}

impl From<u8> for ItemType {
    fn from(pos: u8) -> Self {
        match pos {
            1 => ItemType::MiscObject,
            2 => ItemType::Chest,
            3 => ItemType::MiscUsable,
            4 => ItemType::Jewelry,
            5 => ItemType::Gem,
            6 => ItemType::Bag,
            7 => ItemType::WearableGem,

            10 => ItemType::SlingAmmo,
            11 => ItemType::Bolt,
            12 => ItemType::Arrow,
            13 => ItemType::Spike,

            15 => ItemType::LightSource,

            16 => ItemType::Bow,
            17 => ItemType::Crossbow,
            18 => ItemType::Sling,

            21 => ItemType::Axe,
            22 => ItemType::Polearm,
            23 => ItemType::Dagger,
            24 => ItemType::Sword,
            25 => ItemType::Pick,
            26 => ItemType::Mace,

            30 => ItemType::Boots,
            31 => ItemType::Gloves,
            32 => ItemType::Cloak,
            33 => ItemType::Helm,
            34 => ItemType::Shield,
            35 => ItemType::HardArmor,
            36 => ItemType::SoftArmor,
            37 => ItemType::Bracers,
            38 => ItemType::Belt,

            40 => ItemType::Amulet,

            45 => ItemType::Ring,

            55 => ItemType::Staff,

            65 => ItemType::Wand,
            70 => ItemType::Scroll,

            75 => ItemType::Potion,
            77 => ItemType::FlaskOfOil,

            80 => ItemType::Food,
            81 => ItemType::JunkFood,

            85 => ItemType::Chime,
            86 => ItemType::Horn,

            90 => ItemType::MagicBook,
            91 => ItemType::PrayerBook,
            92 => ItemType::Instrument,
            93 => ItemType::SongBook,

            95 => ItemType::LodgingAtInn,
            100 => ItemType::Money,
            101 => ItemType::UnseenTrap,
            102 => ItemType::SeenTrap,
            103 => ItemType::Rubble,
            104 => ItemType::OpenDoor,
            105 => ItemType::ClosedDoor,
            107 => ItemType::UpStaircase,
            108 => ItemType::DownStaircase,
            109 => ItemType::SecretDoor,
            110 => ItemType::EntranceToStore,
            111 => ItemType::UpSteepStaircase,
            112 => ItemType::DownSteepStaircase,
            113 => ItemType::Whirlpool,
            _ => panic!(),
        }
    }
}
