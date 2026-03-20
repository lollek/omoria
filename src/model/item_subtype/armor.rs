use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GemHelmSubType {
    IronHelm,
    SteelHelm,
}

impl From<GemHelmSubType> for usize {
    fn from(value: GemHelmSubType) -> usize {
        match value {
            GemHelmSubType::IronHelm => 9,
            GemHelmSubType::SteelHelm => 10,
        }
    }
}

impl TryFrom<usize> for GemHelmSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            9 => Ok(GemHelmSubType::IronHelm),
            10 => Ok(GemHelmSubType::SteelHelm),
            _ => Err(()),
        }
    }
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

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::GemHelmSubType;

    #[test]
    fn test_gem_helm_subtype_try_from_usize_accepts_known_values() {
        assert_eq!(
            GemHelmSubType::try_from(9usize).unwrap(),
            GemHelmSubType::IronHelm
        );
        assert_eq!(
            GemHelmSubType::try_from(10usize).unwrap(),
            GemHelmSubType::SteelHelm
        );
    }

    #[test]
    fn test_gem_helm_subtype_try_from_usize_rejects_unknown_values() {
        assert!(GemHelmSubType::try_from(8usize).is_err());
        assert!(GemHelmSubType::try_from(11usize).is_err());
    }

    #[test]
    fn test_gem_helm_subtype_into_usize_returns_expected_codes() {
        assert_eq!(usize::from(GemHelmSubType::IronHelm), 9);
        assert_eq!(usize::from(GemHelmSubType::SteelHelm), 10);
    }
}
