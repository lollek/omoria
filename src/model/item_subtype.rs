use crate::model::ItemType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemSubType {
    MiscObject(MiscObjectSubType),
    Chest(ChestSubType),
    MiscUsable(MiscUsableSubType),
    Jewelry(JewelrySubType),
    Gem(GemSubType),
    Bag(BagSubType),
    WearableGem(WearableGemSubType),

    SlingAmmo(SlingAmmoSubType),
    Bolt(BoltSubType),
    Arrow(ArrowSubType),
    Spike(SpikeSubType),

    LightSource(LightSourceSubType),

    RangedWeapon(RangedWeaponSubType),
    HaftedWeapon(HaftedWeaponSubType),
    PoleArm(PoleArmSubType),
    Dagger(DaggerSubType),
    Sword(SwordSubType),
    Pick(PickSubType),
    Maul(MaulSubType),

    GemHelm(GemHelmSubType),
    Boots(BootsSubType),
    Gloves(GlovesSubType),
    Cloak(CloakSubType),
    Helm(HelmSubType),
    Shield(ShieldSubType),
    HardArmor(HardArmorSubType),
    SoftArmor(SoftArmorSubType),
    Bracers(BracersSubType),
    Belt(BeltSubType),

    Amulet(AmuletSubType),

    Ring(RingSubType),

    Staff(StaffSubType),

    Rod(RodSubType),

    Wand(WandSubType),
    Scroll1(Scroll1SubType),
    Scroll2(Scroll2SubType),

    Potion1(Potion1SubType),
    Potion2(Potion2SubType),
    FlaskOfOil(FlaskOfOilSubType),

    Food(FoodSubType),
    JunkFood(JunkFoodSubType),

    Chime(ChimeSubType),
    Horn(HornSubType),

    MagicBook(MagicBookSubType),
    PrayerBook(PrayerBookSubType),
    Instrument(InstrumentSubType),
    SongBook(SongBookSubType),

    LodgingAtInn(LodgingAtInnSubType),
}

impl ItemSubType {
    pub fn get_type(&self) -> ItemType {
        match self {
            ItemSubType::MiscObject(_) => ItemType::MiscObject,
            ItemSubType::Chest(_) => ItemType::Chest,
            ItemSubType::MiscUsable(_) => ItemType::MiscUsable,
            ItemSubType::Jewelry(_) => ItemType::Jewelry,
            ItemSubType::Gem(_) => ItemType::Gem,
            ItemSubType::Bag(_) => ItemType::Bag,
            ItemSubType::WearableGem(_) => ItemType::WearableGem,
            ItemSubType::SlingAmmo(_) => ItemType::SlingAmmo,
            ItemSubType::Bolt(_) => ItemType::Bolt,
            ItemSubType::Arrow(_) => ItemType::Arrow,
            ItemSubType::Spike(_) => ItemType::Spike,
            ItemSubType::LightSource(_) => ItemType::LightSource,
            ItemSubType::RangedWeapon(_) => ItemType::RangedWeapon,
            ItemSubType::HaftedWeapon(_) => ItemType::HaftedWeapon,
            ItemSubType::PoleArm(_) => ItemType::PoleArm,
            ItemSubType::Dagger(_) => ItemType::Dagger,
            ItemSubType::Sword(_) => ItemType::Sword,
            ItemSubType::Pick(_) => ItemType::Pick,
            ItemSubType::Maul(_) => ItemType::Maul,
            ItemSubType::GemHelm(_) => ItemType::GemHelm,
            ItemSubType::Boots(_) => ItemType::Boots,
            ItemSubType::Gloves(_) => ItemType::Gloves,
            ItemSubType::Cloak(_) => ItemType::Cloak,
            ItemSubType::Helm(_) => ItemType::Helm,
            ItemSubType::Shield(_) => ItemType::Shield,
            ItemSubType::HardArmor(_) => ItemType::HardArmor,
            ItemSubType::SoftArmor(_) => ItemType::SoftArmor,
            ItemSubType::Bracers(_) => ItemType::Bracers,
            ItemSubType::Belt(_) => ItemType::Belt,
            ItemSubType::Amulet(_) => ItemType::Amulet,
            ItemSubType::Ring(_) => ItemType::Ring,
            ItemSubType::Staff(_) => ItemType::Staff,
            ItemSubType::Rod(_) => ItemType::Rod,
            ItemSubType::Wand(_) => ItemType::Wand,
            ItemSubType::Scroll1(_) => ItemType::Scroll1,
            ItemSubType::Scroll2(_) => ItemType::Scroll2,
            ItemSubType::Potion1(_) => ItemType::Potion1,
            ItemSubType::Potion2(_) => ItemType::Potion2,
            ItemSubType::FlaskOfOil(_) => ItemType::FlaskOfOil,
            ItemSubType::Food(_) => ItemType::Food,
            ItemSubType::JunkFood(_) => ItemType::JunkFood,
            ItemSubType::Chime(_) => ItemType::Chime,
            ItemSubType::Horn(_) => ItemType::Horn,
            ItemSubType::MagicBook(_) => ItemType::MagicBook,
            ItemSubType::PrayerBook(_) => ItemType::PrayerBook,
            ItemSubType::Instrument(_) => ItemType::Instrument,
            ItemSubType::SongBook(_) => ItemType::SongBook,
            ItemSubType::LodgingAtInn(_) => ItemType::LodgingAtInn,
        }
    }
}

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiscObjectSubType {
    RatSkeleton,
    GiantCentipedeSkeleton,
    EmptyBottle,
    PotteryShard,
    HumanSkeleton,
    DwarfSkeleton,
    ElfSkeleton,
    GnomeSkeleton,
    BrokenTeeth,
    LargeBrokenBone,
    BrokenStick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChestSubType {
    //  DeadHumanBody,
    SmallWoodenChest,
    LargeWoodenChest,
    SmallIronChest,
    LargeIronChest,
    SmallSteelChest,
    LargeSteelChest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MiscUsableSubType {
    FlaskOfOil,
    IronSpike,
    Statue,
    SilverCross,
    GoldCross,
    MithrilCross,
    Cross,
    CorkedBottle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JewelrySubType {
    SmallGoldPendant,
    SmallMithrilPendant,
    LargeMithrilGarterBelt,
    SmallSilverPendant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GemSubType {
    GemOfDetectMonsters,
    GemOfDispelEvil,
    GemOfDarkness,
    GemOfAcidBalls,
    GemOfDetectInvisible,
    GemOfIdentify,
    GemOfLight,
    GemOfSummoning,
    GemOfRemoveCurse,
    GemOfAnnihilation,
    GemOfRecall,
    FineAgate,
    FineDiamond,
    RoughDiamond,
    RoughSapphire,
    FineSapphire,
    SmallBagOfOpals,
    SmallBagOfSapphires,
    SmallPouchOfDiamonds,
    LargeSackOfPearls,
    LargeSackOfSapphires,
    LargePouchOfDiamonds,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BagSubType {
    BagOfHolding250,
    BagOfHolding500,
    BagOfHolding1000,
    BagOfHolding1500,
    BagOfDevouring,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WearableGemSubType {
    GemOfTeleportation,
    GemOfResistCold,
    GemOfResistAcid,
    GemOfSeeInvisible,
    GemOfStealth,
    GemOfSlowDigestion,
    GemOfProtectFire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SlingAmmoSubType {
    RoundedPebble,
    IronShot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BoltSubType {
    Bolt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArrowSubType {
    Arrow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpikeSubType {
    IronSpike,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LightSourceSubType {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RangedWeaponSubType {
    Shortbow,
    HuntersBow,
    CompositeBow,
    WarBow,
    DoubleBow,
    SiegeBow,
    WardedBow,
    SiegeCrossbow,
    Ballista,
    LightCrossbow,
    HeavyCrossbow,
    Sling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HaftedWeaponSubType {
    Balestarius,
    BattleAxe,
    BroadAxe,
    HandAxe,
    WarAxe,
    LargeAxe,
    BeardedAxe,
    SilverEdgedAxe,
    ChampionAxe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PoleArmSubType {
    AwlPike,
    BeakedAxe,
    Fauchard,
    Glaive,
    Halberd,
    LucerneHammer,
    Pike,
    Spike,
    Lance,
    Javelin,
    Naginata,
    WarScythe,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DaggerSubType {
    MainGauche,
    Misercorde,
    Stiletto,
    Bodkin,
    BrokenDagger,
    CatONineTails,
    Bilbo,
    Baselard,
    Foil,
    Rapier,
    SmallSword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SwordSubType {
    Backsword,
    BastardSword,
    Broadsword,
    Claymore,
    Cutlass,
    Espadon,
    ExecutionersSword,
    Flamberge,
    Katana,
    Longsword,
    Nodachi,
    Sabre,
    Zweihander,
    BrokenSword,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PickSubType {
    Pick,
    Shovel,
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MaulSubType {
    BallAndChain,
    WoodenClub,
    Flail,
    GreatFlail,
    MorningStar,
    Mace,
    WarHammer,
    LeadFilledMace,
    IronShodQuarterstaff,
    OgreMaul,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GemHelmSubType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BootsSubType {
    SoftLeatherShoes,
    SoftLeatherBoots,
    HardLeatherBoots,
    Sandals,
    ChainBoots,
    LightPlatedBoots,
    SharkskinBoots,
    DemonhideBoots,
    WyrmhideBoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GlovesSubType {
    LeatherGloves,
    HeavyGloves,
    ClothGloves,
    ChainGloves,
    LightGauntlets,
    HeavyGauntlets,
    SharkskinGloves,
    WarGauntlets,
    DemonhideGloves,
    WyrmhideGloves,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloakSubType {
    LightCloak,
    HeavyCloak,
    SharkskinCloak,
    DemonhideCloak,
    WyrmhideCloak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HelmSubType {
    ClothHat,
    SoftLeatherCap,
    HardLeatherCap,
    MetalCap,
    FullHelm,
    GreatHelm,
    WingedHelm,
    SilverCrown,
    SilverMask,
    GoldenCrown,
    GoldenMask,
    JewelEncrustedCrown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShieldSubType {
    SmallLeatherShield,
    MediumLeatherShield,
    LargeLeatherShield,
    Buckler,
    KiteShield,
    TowerShield,
    SharkskinShield,
    DemonhideShield,
    WyrmhideShield,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HardArmorSubType {
    AugmentedChainMail,
    BarChainMail,
    BronzePlateMail,
    ChainMail,
    DoubleChainMail,
    FullPlateArmor,
    LacqueredPlate,
    LaminatedArmor,
    MetalBrigandineArmor,
    MetalLamellarArmor,
    MetalScaleMail,
    MithrilChainMail,
    MithrilPlateArmor,
    PartialPlateArmor,
    RustyChainMail,
    StonePlateArmor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SoftArmorSubType {
    CoolSetOfThreads,
    DemonhideArmor,
    DuskShroud,
    ElvenChainMail,
    FilthyNagaHideArmor,
    FilthyRags,
    HardLeatherArmor,
    HardLeatherRingMail,
    HardStuddedLeather,
    LeatherScaleMail,
    Robe,
    SoftLeatherArmor,
    SoftLeatherRingMail,
    SoftStuddedLeather,
    WovenCordArmor,
    WyrmhideArmor,
    LeatherBrigantineArmor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BracersSubType {
    BracersOfProtection,
    BracersOfDefense,
    BracersOfShielding,
    MithrilBracers,
    AdamantiteBracers,
    BracersOfWeaponAttraction,
    SilverBraceletOfWarding,
    SilverBracelet,
    GoldBracelet,
    PlatinumBracelet,
    LeatherBracers,
    StuddedLeatherBracers,
    LightPlatedBracers,
    SharkskinBracers,
    DemonhideBracers,
    WyrmhideBracers,
    ChainmailBracers,
    LamellarBracers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BeltSubType {
    Sash,
    LightBelt,
    Belt,
    HeavyBelt,
    LightPlatedBelt,
    SharkskinBelt,
    DemonhideBelt,
    WyrmhideBelt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AmuletSubType {
    AmuletOfAdornment1,
    AmuletOfAdornment2,
    AmuletOfWisdom,
    AmuletOfCharisma,
    AmuletOfSearching,
    AmuletOfTeleportation,
    AmuletOfSlowDigestion,
    AmuletOfResistAcid,
    AmuletOfTheMagi,
    AmuletOfDoom,
    SilverNecklace,
    GoldNecklace,
    MithrilNecklace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RingSubType {
    RingOfGainStrength,
    RingOfGainDexterity,
    RingOfGainConstitution,
    RingOfGainIntelligence,
    RingOfSpeed1,
    RingOfSpeed2,
    RingOfSearching,
    RingOfTeleportation,
    RingOfSlowDigestion,
    RingOfResistFire,
    RingOfResistCold,
    RingOfFeatherFalling,
    RingOfAdornment1,
    RingOfAdornment2,
    RingOfWeakness,
    RingOfLordlyProtectionFire,
    RingOfLordlyProtectionAcid,
    RingOfLordlyProtectionCold,
    RingOfWoe,
    RingOfStupidity,
    RingOfIncreaseDamage,
    RingOfIncreaseToHit,
    RingOfProtection,
    RingOfAggravateMonsters,
    RingOfSeeInvisible,
    RingOfSustainStrength,
    RingOfSustainIntelligence,
    RingOfSustainWisdom,
    RingOfSustainConstitution,
    RingOfSustainDexterity,
    RingOfSustainCharisma,
    RingOfSlaying,
    RingOfGnomekind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StaffSubType {
    StaffOfLight,
    StaffOfDoorStairLocation,
    StaffOfTrapLocation,
    StaffOfTreasureLocation,
    StaffOfObjectLocation,
    StaffOfTeleportation,
    StaffOfEarthquakes,
    StaffOfSummoning,
    StaffOfDestruction,
    StaffOfStarlite,
    StaffOfHasteMonsters,
    StaffOfSlowMonsters,
    StaffOfSleepMonsters,
    StaffOfCureLightWounds,
    StaffOfDetectInvisible,
    StaffOfSpeed,
    StaffOfSlowness,
    StaffOfMassPolymorph,
    StaffOfRemoveCurse,
    StaffOfDetectEvil,
    StaffOfCuring,
    StaffOfDispelEvil,
    StaffOfDarkness,
    StaffOfIdentify,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RodSubType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WandSubType {
    WandOfProbing,
    WandOfLight,
    WandOfLightningBolts,
    WandOfFrostBolts,
    WandOfFireBolts,
    WandOfStoneToMud,
    WandOfPolymorph,
    WandOfHealMonster,
    WandOfHasteMonster,
    WandOfSlowMonster,
    WandOfConfuseMonster,
    WandOfSleepMonster,
    WandOfDrainLife,
    WandOfTrapDoorDestruction,
    WandOfMagicMissile,
    WandOfWallBuilding,
    WandOfCloneMonster,
    WandOfTeleportAway,
    WandOfDisarming,
    WandOfLightningBalls,
    WandOfColdBalls,
    WandOfFireBalls,
    WandOfStinkingCloud,
    WandOfAcidBalls,
    WandOfWonder,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scroll1SubType {
    AggravateMonster,
    Blessing,
    CreateFood,
    CurseArmor,
    CurseWeapon,
    Darkness,
    Destruction,
    DetectInvisible,
    DispelUndead,
    DoorCreation,
    DoorStairLocation,
    EnchantArmor,
    EnchantWeapon,
    EnchantWeaponToDam,
    EnchantWeaponToHit,
    FeignDeath,
    Genocide,
    HolyChant,
    HolyPrayer,
    Identify,
    Light,
    MagicMapping,
    MakeMunchies,
    MassGenocide,
    MonsterConfusion,
    ObjectDetection,
    PhaseDoor,
    ProtectionFromEvil,
    Recharging,
    RemoveCurse,
    RuneOfProtection,
    SleepMonster,
    SummonMonster,
    SummonUndead,
    Teleport,
    TeleportLevel,
    TrapCreation,
    TrapDetection,
    TrapDoorDestruction,
    TreasureDetection,
    Wishing,
    WordOfRecall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scroll2SubType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Potion1SubType {
    AppleJuice,
    Blindness,
    Boldliness,
    Charisma,
    Confusion,
    CureCriticalWounds,
    CureLightWounds,
    CureSeriousWounds,
    DetectInvisible,
    FleaBile,
    GainConstitution,
    GainDexterity,
    GainExperience,
    GainIntelligence,
    GainStrength,
    GainWisdom,
    HasteSelf,
    Healing,
    Heroism,
    InfraVision,
    Invulnerability,
    Learning,
    LoseIntelligence,
    LoseMemories,
    LoseWisdom,
    NeutralizePoison,
    Poison,
    ResistCold,
    ResistHeat,
    RestoreCharisma,
    RestoreConstitution,
    RestoreDexterity,
    RestoreIntelligence,
    RestoreLifeLevels,
    RestoreMana,
    RestoreStrength,
    RestoreWisdom,
    SaltWater,
    Sleep,
    SlimeMoldJuice,
    SlowPoison,
    Slowness,
    SuperHeroism,
    Ugliness,
    Water,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Potion2SubType {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FlaskOfOilSubType {
    FlaskOfOil,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FoodSubType {
    Mushroom,
    MushroomOfPoison,
    MushroomOfBlindness,
    MushroomOfParanoia,
    MushroomOfConfusion,
    MushroomOfHallucination,
    MushroomOfCurePoison,
    MushroomOfCureBlindness,
    MushroomOfCureParanoia,
    MushroomOfCureConfusion,
    MushroomOfWeakness,
    MushroomOfUnhealth,
    MushroomOfRestoreConstitution,
    MushroomOfFirstAid,
    MushroomOfMinorCures,
    MushroomOfLightCures,
    MushroomOfRestoring,
    MushroomOfPoison2,
    MushroomOfHallucination2,
    MushroomOfCurePoison2,
    MushroomOfUnhealth2,
    MushroomOfCureSeriousWounds,
    PintOfFineGradeMush,
    RationOfFood,
    Mushroom2,
    HardBiscuit,
    BeefJerky,
    FineAle,
    FineWine,
    ElvishWaybread,
    Stew,
    GreenJelly,
    BerriesPoisonous,
    BerriesSmurfberries,
    BerriesGoodberries,
    EyeballOfNed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum JunkFoodSubType {
    BoxOfPiranhaCrackers,
    CanOfOrcaCola,
    TwelvePoundTrollBuger,
    BagOfBrontosaurusChips,
    SliceOfPurpleMushroomPizza,
    PeanutButterAndGrapeJellySandwich,
    DragonSteak,
    VorpalBunnyThroatLozenge,
    DeepFriedGiantCentipede,
    PintOfBeetleJuice,
    BownOfBatStew,
    JarOfPickledLeeches,
    PackOfKittenMcNuggets,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ChimeSubType {
    ChimeOfLight,
    ChimeOfDetectDoorsStairs,
    ChimeOfDetectTraps,
    ChimeOfTeleportation,
    ChimeOfThunderblast,
    ChimeOfSummonMonster,
    ChimeOfDisarming,
    ChimeOfAggravation,
    ChimeOfSlowMonster,
    ChimeOfSootheMonster,
    ChimeOfCureLightWound,
    ChimeOfChanging,
    ChimeOfRemoveCurse,
    ChimeOfCuring,
    ChimeOfDispelEvil,
    ChimeOfDarkness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HornSubType {
    HornOfBubbles,
    HornOfCalling,
    HornOfSoftSounds,
    HornOfBlasting,
    HornOfCold,
    HornOfHeat,
    HornOfGas,
    HornOfRecall,
    HornOfChaos,
    HornOfGlue,
    HornOfValhalla,
    HornOfTritons,
    HornOfFog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MagicBookSubType {
    BeginnersMagic,
    Magic1,
    Magic2,
    MagesGuideToPower,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrayerBookSubType {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum InstrumentSubType {
    PipesOfPeace,
    LyreOfNature,
    LuteOfTheWoods,
    HarpOfTheDruids,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SongBookSubType {
    BeginnersHandbook,
    SongBook1,
    SongBook2,
    GreaterSongBook,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LodgingAtInnSubType {
    LodgingForOneDay,
    LodgingForThreeDays,
    LodgingForOneWeek,
    RoomAndBoardForOneDay,
}
