#[derive(Debug, PartialEq, Eq)]
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
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum ChestSubType {
    SmallWoodenChest,
    LargeWoodenChest,
    SmallIronChest,
    LargeIronChest,
    SmallSteelChest,
    LargeSteelChest,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum JewelrySubType {
    SmallGoldPendant,
    SmallMithrilPendant,
    LargeMithrilGarterBelt,
    SmallSilverPendant,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum BagSubType {
    BagOfHolding250,
    BagOfHolding500,
    BagOfHolding1000,
    BagOfHolding1500,
    BagOfDevouring,
}

#[derive(Debug, PartialEq, Eq)]
pub enum WearableGemSubType {
    GemOfTeleportation,
    GemOfResistCold,
    GemOfResistAcid,
    GemOfSeeInvisible,
    GemOfStealth,
    GemOfSlowDigestion,
    GemOfProtectFire,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SlingAmmoSubType {
    RoundedPebble,
    IronShot,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoltSubType {
    Bolt,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArrowSubType {
    Arrow,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SpikeSubType {
    IronSpike,
}

#[derive(Debug, PartialEq, Eq)]
pub enum LightSourceSubType {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum PickSubType {
    Pick,
    Shovel,
    OrcishPick1,
    OrcishPick2,
    DwarvenPick,
    GnomishShovel,
    DwarvenShovel,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum GemHelmSubType {}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum CloakSubType {
    LightCloak,
    HeavyCloak,
    SharkskinCloak,
    DemonhideCloak,
    WyrmhideCloak,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum RodSubType {}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Scroll2SubType {}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum Potion2SubType {}

#[derive(Debug, PartialEq, Eq)]
pub enum FlaskOfOilSubType {
    FlaskOfOil,
}

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
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

#[derive(Debug, PartialEq, Eq)]
pub enum MagicBookSubType {
    BeginnersMagic,
    Magic1,
    Magic2,
    MagesGuideToPower,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrayerBookSubType {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

#[derive(Debug, PartialEq, Eq)]
pub enum InstrumentSubType {
    PipesOfPeace,
    LyreOfNature,
    LuteOfTheWoods,
    HarpOfTheDruids,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SongBookSubType {
    BeginnersHandbook,
    SongBook1,
    SongBook2,
    GreaterSongBook,
}
