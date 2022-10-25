#[derive(Debug, PartialEq, Eq)]
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
