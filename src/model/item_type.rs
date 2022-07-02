use item_template;

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum ItemType {
    MiscObject(item_template::MiscTemplate),
    Chest(item_template::ChestTemplate),
    MiscUsable(item_template::MiscUsableTemplate),
    Jewelry(item_template::JewelryTemplate),
    Gem(item_template::GemTemplate),
    Bag(item_template::BagTemplate),
    WearableGem(item_template::WearableGemTemplate),

    SlingAmmo(item_template::AmmunitionTemplate),
    Bolt(item_template::AmmunitionTemplate),
    Arrow(item_template::AmmunitionTemplate),
    Spike(item_template::MiscUsableTemplate),

    LightSource(item_template::LightSourceTemplate),

    Bow(item_template::BowTemplate),
    Crossbow(item_template::CrossbowTemplate),
    Sling(item_template::SlingTemplate),

    Axe(item_template::AxeTemplate),
    Polearm(item_template::PolearmTemplate),
    Dagger(item_template::DaggerTemplate),
    Sword(item_template::SwordTemplate),
    Pick(item_template::PickTemplate),
    Mace(item_template::MaceTemplate),

    Boots(item_template::BootsTemplate),
    Gloves(item_template::GlovesTemplate),
    Cloak(item_template::CloakTemplate),
    Helm(item_template::HelmTemplate),
    Shield(item_template::ShieldTemplate),
    HardArmor(item_template::HardArmorTemplate),
    SoftArmor(item_template::SoftArmorTemplate),
    Bracers(item_template::BracersTemplate),
    Belt(item_template::BeltTemplate),

    Amulet(item_template::AmuletTemplate),

    Ring(item_template::RingTemplate),

    Staff(item_template::StaffTemplate),

    Wand(item_template::WandTemplate),
    Scroll(item_template::ScrollTemplate),

    Potion(item_template::PotionTemplate),
    FlaskOfOil(item_template::MiscUsableTemplate),

    Food(item_template::FoodTemplate),
    JunkFood(item_template::JunkFoodTemplate),

    Chime(item_template::ChimeTemplate),
    Horn(item_template::HornTemplate),

    MagicBook(item_template::MagicBookTemplate),
    PrayerBook(item_template::PrayerBookTemplate),
    Instrument(item_template::InstrumentTemplate),
    SongBook(item_template::SongBookTemplate),

    // Not Items, but yeah
    LodgingAtInn(item_template::LodgingAtInnTemplate),
    Money(item_template::DungeonFeatureTemplate),
    UnseenTrap(item_template::DungeonFeatureTemplate),
    SeenTrap(item_template::DungeonFeatureTemplate),
    Rubble(item_template::DungeonFeatureTemplate),
    OpenDoor(item_template::DungeonFeatureTemplate),
    ClosedDoor(item_template::DungeonFeatureTemplate),
    UpStaircase(item_template::DungeonFeatureTemplate),
    DownStaircase(item_template::DungeonFeatureTemplate),
    SecretDoor(item_template::DungeonFeatureTemplate),
    EntranceToStore(item_template::DungeonFeatureTemplate),
    UpSteepStaircase(item_template::DungeonFeatureTemplate),
    DownSteepStaircase(item_template::DungeonFeatureTemplate),
    Whirlpool(item_template::DungeonFeatureTemplate),
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
