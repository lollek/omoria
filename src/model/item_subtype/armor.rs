#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GemHelmSubType {
    IronHelm,
    SteelHelm,
}

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
