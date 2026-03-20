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

impl From<BootsSubType> for usize {
    fn from(value: BootsSubType) -> usize {
        match value {
            BootsSubType::SoftLeatherShoes => 1,
            BootsSubType::SoftLeatherBoots => 2,
            BootsSubType::HardLeatherBoots => 3,
            BootsSubType::Sandals => 4,
            BootsSubType::ChainBoots => 5,
            BootsSubType::LightPlatedBoots => 6,
            BootsSubType::SharkskinBoots => 7,
            BootsSubType::DemonhideBoots => 8,
            BootsSubType::WyrmhideBoot => 9,
        }
    }
}

impl TryFrom<usize> for BootsSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BootsSubType::SoftLeatherShoes),
            2 => Ok(BootsSubType::SoftLeatherBoots),
            3 => Ok(BootsSubType::HardLeatherBoots),
            4 => Ok(BootsSubType::Sandals),
            5 => Ok(BootsSubType::ChainBoots),
            6 => Ok(BootsSubType::LightPlatedBoots),
            7 => Ok(BootsSubType::SharkskinBoots),
            8 => Ok(BootsSubType::DemonhideBoots),
            9 => Ok(BootsSubType::WyrmhideBoot),
            _ => Err(()),
        }
    }
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

impl From<GlovesSubType> for usize {
    fn from(value: GlovesSubType) -> usize {
        match value {
            GlovesSubType::LeatherGloves => 1,
            GlovesSubType::HeavyGloves => 2,
            GlovesSubType::ClothGloves => 5,
            GlovesSubType::ChainGloves => 6,
            GlovesSubType::LightGauntlets => 7,
            GlovesSubType::HeavyGauntlets => 8,
            GlovesSubType::SharkskinGloves => 9,
            GlovesSubType::WarGauntlets => 10,
            GlovesSubType::DemonhideGloves => 11,
            GlovesSubType::WyrmhideGloves => 12,
        }
    }
}

impl TryFrom<usize> for GlovesSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(GlovesSubType::LeatherGloves),
            2 => Ok(GlovesSubType::HeavyGloves),
            5 => Ok(GlovesSubType::ClothGloves),
            6 => Ok(GlovesSubType::ChainGloves),
            7 => Ok(GlovesSubType::LightGauntlets),
            8 => Ok(GlovesSubType::HeavyGauntlets),
            9 => Ok(GlovesSubType::SharkskinGloves),
            10 => Ok(GlovesSubType::WarGauntlets),
            11 => Ok(GlovesSubType::DemonhideGloves),
            12 => Ok(GlovesSubType::WyrmhideGloves),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CloakSubType {
    LightCloak,
    HeavyCloak,
    SharkskinCloak,
    DemonhideCloak,
    WyrmhideCloak,
}

impl From<CloakSubType> for usize {
    fn from(value: CloakSubType) -> usize {
        match value {
            CloakSubType::LightCloak => 1,
            CloakSubType::HeavyCloak => 2,
            CloakSubType::SharkskinCloak => 3,
            CloakSubType::DemonhideCloak => 4,
            CloakSubType::WyrmhideCloak => 5,
        }
    }
}

impl TryFrom<usize> for CloakSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CloakSubType::LightCloak),
            2 => Ok(CloakSubType::HeavyCloak),
            3 => Ok(CloakSubType::SharkskinCloak),
            4 => Ok(CloakSubType::DemonhideCloak),
            5 => Ok(CloakSubType::WyrmhideCloak),
            _ => Err(()),
        }
    }
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

impl From<HelmSubType> for usize {
    fn from(value: HelmSubType) -> usize {
        match value {
            HelmSubType::ClothHat => 12,
            HelmSubType::SoftLeatherCap => 13,
            HelmSubType::HardLeatherCap => 14,
            HelmSubType::MetalCap => 15,
            HelmSubType::FullHelm => 16,
            HelmSubType::GreatHelm => 17,
            HelmSubType::WingedHelm => 18,
            HelmSubType::SilverCrown => 19,
            HelmSubType::SilverMask => 20,
            HelmSubType::GoldenCrown => 21,
            HelmSubType::GoldenMask => 22,
            HelmSubType::JewelEncrustedCrown => 23,
        }
    }
}

impl TryFrom<usize> for HelmSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            12 => Ok(HelmSubType::ClothHat),
            13 => Ok(HelmSubType::SoftLeatherCap),
            14 => Ok(HelmSubType::HardLeatherCap),
            15 => Ok(HelmSubType::MetalCap),
            16 => Ok(HelmSubType::FullHelm),
            17 => Ok(HelmSubType::GreatHelm),
            18 => Ok(HelmSubType::WingedHelm),
            19 => Ok(HelmSubType::SilverCrown),
            20 => Ok(HelmSubType::SilverMask),
            21 => Ok(HelmSubType::GoldenCrown),
            22 => Ok(HelmSubType::GoldenMask),
            23 => Ok(HelmSubType::JewelEncrustedCrown),
            _ => Err(()),
        }
    }
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

impl From<ShieldSubType> for usize {
    fn from(value: ShieldSubType) -> usize {
        match value {
            ShieldSubType::SmallLeatherShield => 1,
            ShieldSubType::MediumLeatherShield => 2,
            ShieldSubType::LargeLeatherShield => 3,
            ShieldSubType::Buckler => 4,
            ShieldSubType::KiteShield => 5,
            ShieldSubType::TowerShield => 6,
            ShieldSubType::SharkskinShield => 7,
            ShieldSubType::DemonhideShield => 8,
            ShieldSubType::WyrmhideShield => 9,
        }
    }
}

impl TryFrom<usize> for ShieldSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ShieldSubType::SmallLeatherShield),
            2 => Ok(ShieldSubType::MediumLeatherShield),
            3 => Ok(ShieldSubType::LargeLeatherShield),
            4 => Ok(ShieldSubType::Buckler),
            5 => Ok(ShieldSubType::KiteShield),
            6 => Ok(ShieldSubType::TowerShield),
            7 => Ok(ShieldSubType::SharkskinShield),
            8 => Ok(ShieldSubType::DemonhideShield),
            9 => Ok(ShieldSubType::WyrmhideShield),
            _ => Err(()),
        }
    }
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

impl From<HardArmorSubType> for usize {
    fn from(value: HardArmorSubType) -> usize {
        match value {
            HardArmorSubType::AugmentedChainMail => 5,
            HardArmorSubType::BarChainMail => 6,
            HardArmorSubType::BronzePlateMail => 13,
            HardArmorSubType::ChainMail => 2,
            HardArmorSubType::DoubleChainMail => 4,
            HardArmorSubType::FullPlateArmor => 11,
            HardArmorSubType::LacqueredPlate => 12,
            HardArmorSubType::LaminatedArmor => 8,
            HardArmorSubType::MetalBrigandineArmor => 7,
            HardArmorSubType::MetalLamellarArmor => 10,
            HardArmorSubType::MetalScaleMail => 1,
            HardArmorSubType::MithrilChainMail => 15,
            HardArmorSubType::MithrilPlateArmor => 16,
            HardArmorSubType::PartialPlateArmor => 9,
            HardArmorSubType::RustyChainMail => 3,
            HardArmorSubType::StonePlateArmor => 14,
        }
    }
}

impl TryFrom<usize> for HardArmorSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            5 => Ok(HardArmorSubType::AugmentedChainMail),
            6 => Ok(HardArmorSubType::BarChainMail),
            13 => Ok(HardArmorSubType::BronzePlateMail),
            2 => Ok(HardArmorSubType::ChainMail),
            4 => Ok(HardArmorSubType::DoubleChainMail),
            11 => Ok(HardArmorSubType::FullPlateArmor),
            12 => Ok(HardArmorSubType::LacqueredPlate),
            8 => Ok(HardArmorSubType::LaminatedArmor),
            7 => Ok(HardArmorSubType::MetalBrigandineArmor),
            10 => Ok(HardArmorSubType::MetalLamellarArmor),
            1 => Ok(HardArmorSubType::MetalScaleMail),
            15 => Ok(HardArmorSubType::MithrilChainMail),
            16 => Ok(HardArmorSubType::MithrilPlateArmor),
            9 => Ok(HardArmorSubType::PartialPlateArmor),
            3 => Ok(HardArmorSubType::RustyChainMail),
            14 => Ok(HardArmorSubType::StonePlateArmor),
            _ => Err(()),
        }
    }
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

impl From<SoftArmorSubType> for usize {
    fn from(value: SoftArmorSubType) -> usize {
        match value {
            SoftArmorSubType::CoolSetOfThreads => 11,
            SoftArmorSubType::DemonhideArmor => 15,
            SoftArmorSubType::DuskShroud => 14,
            SoftArmorSubType::ElvenChainMail => 13,
            SoftArmorSubType::FilthyNagaHideArmor => 12,
            SoftArmorSubType::FilthyRags => 99,
            SoftArmorSubType::HardLeatherArmor => 4,
            SoftArmorSubType::HardLeatherRingMail => 8,
            SoftArmorSubType::HardStuddedLeather => 5,
            SoftArmorSubType::LeatherScaleMail => 9,
            SoftArmorSubType::Robe => 1,
            SoftArmorSubType::SoftLeatherArmor => 2,
            SoftArmorSubType::SoftLeatherRingMail => 7,
            SoftArmorSubType::SoftStuddedLeather => 3,
            SoftArmorSubType::WovenCordArmor => 6,
            SoftArmorSubType::WyrmhideArmor => 16,
            SoftArmorSubType::LeatherBrigantineArmor => 10,
        }
    }
}

impl TryFrom<usize> for SoftArmorSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            11 => Ok(SoftArmorSubType::CoolSetOfThreads),
            15 => Ok(SoftArmorSubType::DemonhideArmor),
            14 => Ok(SoftArmorSubType::DuskShroud),
            13 => Ok(SoftArmorSubType::ElvenChainMail),
            12 => Ok(SoftArmorSubType::FilthyNagaHideArmor),
            99 => Ok(SoftArmorSubType::FilthyRags),
            4 => Ok(SoftArmorSubType::HardLeatherArmor),
            8 => Ok(SoftArmorSubType::HardLeatherRingMail),
            5 => Ok(SoftArmorSubType::HardStuddedLeather),
            9 => Ok(SoftArmorSubType::LeatherScaleMail),
            1 => Ok(SoftArmorSubType::Robe),
            2 => Ok(SoftArmorSubType::SoftLeatherArmor),
            7 => Ok(SoftArmorSubType::SoftLeatherRingMail),
            3 => Ok(SoftArmorSubType::SoftStuddedLeather),
            6 => Ok(SoftArmorSubType::WovenCordArmor),
            16 => Ok(SoftArmorSubType::WyrmhideArmor),
            10 => Ok(SoftArmorSubType::LeatherBrigantineArmor),
            _ => Err(()),
        }
    }
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

impl From<BracersSubType> for usize {
    fn from(value: BracersSubType) -> usize {
        match value {
            BracersSubType::BracersOfProtection => 1,
            BracersSubType::BracersOfDefense => 2,
            BracersSubType::BracersOfShielding => 3,
            BracersSubType::MithrilBracers => 4,
            BracersSubType::AdamantiteBracers => 5,
            BracersSubType::BracersOfWeaponAttraction => 6,
            BracersSubType::SilverBraceletOfWarding => 31,
            BracersSubType::SilverBracelet => 30,
            BracersSubType::GoldBracelet => 40,
            BracersSubType::PlatinumBracelet => 50,
            BracersSubType::LeatherBracers => 7,
            BracersSubType::StuddedLeatherBracers => 8,
            BracersSubType::LightPlatedBracers => 9,
            BracersSubType::SharkskinBracers => 10,
            BracersSubType::DemonhideBracers => 11,
            BracersSubType::WyrmhideBracers => 12,
            BracersSubType::ChainmailBracers => 13,
            BracersSubType::LamellarBracers => 14,
        }
    }
}

impl TryFrom<usize> for BracersSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BracersSubType::BracersOfProtection),
            2 => Ok(BracersSubType::BracersOfDefense),
            3 => Ok(BracersSubType::BracersOfShielding),
            4 => Ok(BracersSubType::MithrilBracers),
            5 => Ok(BracersSubType::AdamantiteBracers),
            6 => Ok(BracersSubType::BracersOfWeaponAttraction),
            31 => Ok(BracersSubType::SilverBraceletOfWarding),
            30 => Ok(BracersSubType::SilverBracelet),
            40 => Ok(BracersSubType::GoldBracelet),
            50 => Ok(BracersSubType::PlatinumBracelet),
            7 => Ok(BracersSubType::LeatherBracers),
            8 => Ok(BracersSubType::StuddedLeatherBracers),
            9 => Ok(BracersSubType::LightPlatedBracers),
            10 => Ok(BracersSubType::SharkskinBracers),
            11 => Ok(BracersSubType::DemonhideBracers),
            12 => Ok(BracersSubType::WyrmhideBracers),
            13 => Ok(BracersSubType::ChainmailBracers),
            14 => Ok(BracersSubType::LamellarBracers),
            _ => Err(()),
        }
    }
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

impl From<BeltSubType> for usize {
    fn from(value: BeltSubType) -> usize {
        match value {
            BeltSubType::Sash => 1,
            BeltSubType::LightBelt => 2,
            BeltSubType::Belt => 3,
            BeltSubType::HeavyBelt => 4,
            BeltSubType::LightPlatedBelt => 5,
            BeltSubType::SharkskinBelt => 6,
            BeltSubType::DemonhideBelt => 7,
            BeltSubType::WyrmhideBelt => 8,
        }
    }
}

impl TryFrom<usize> for BeltSubType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(BeltSubType::Sash),
            2 => Ok(BeltSubType::LightBelt),
            3 => Ok(BeltSubType::Belt),
            4 => Ok(BeltSubType::HeavyBelt),
            5 => Ok(BeltSubType::LightPlatedBelt),
            6 => Ok(BeltSubType::SharkskinBelt),
            7 => Ok(BeltSubType::DemonhideBelt),
            8 => Ok(BeltSubType::WyrmhideBelt),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use super::{
        BeltSubType, BootsSubType, BracersSubType, CloakSubType, GemHelmSubType, GlovesSubType,
        HardArmorSubType, HelmSubType, ShieldSubType, SoftArmorSubType,
    };

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

    #[test]
    fn test_boots_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            BootsSubType::try_from(1usize).unwrap(),
            BootsSubType::SoftLeatherShoes
        );
    }

    #[test]
    fn test_boots_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(BootsSubType::WyrmhideBoot), 9);
    }

    #[test]
    fn test_gloves_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            GlovesSubType::try_from(1usize).unwrap(),
            GlovesSubType::LeatherGloves
        );
    }

    #[test]
    fn test_gloves_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(GlovesSubType::WyrmhideGloves), 12);
    }

    #[test]
    fn test_cloak_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            CloakSubType::try_from(1usize).unwrap(),
            CloakSubType::LightCloak
        );
    }

    #[test]
    fn test_cloak_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(CloakSubType::WyrmhideCloak), 5);
    }

    #[test]
    fn test_helm_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            HelmSubType::try_from(12usize).unwrap(),
            HelmSubType::ClothHat
        );
    }

    #[test]
    fn test_helm_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(HelmSubType::JewelEncrustedCrown), 23);
    }

    #[test]
    fn test_shield_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            ShieldSubType::try_from(1usize).unwrap(),
            ShieldSubType::SmallLeatherShield
        );
    }

    #[test]
    fn test_shield_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(ShieldSubType::WyrmhideShield), 9);
    }

    #[test]
    fn test_hard_armor_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            HardArmorSubType::try_from(1usize).unwrap(),
            HardArmorSubType::MetalScaleMail
        );
    }

    #[test]
    fn test_hard_armor_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(HardArmorSubType::MithrilPlateArmor), 16);
    }

    #[test]
    fn test_soft_armor_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            SoftArmorSubType::try_from(1usize).unwrap(),
            SoftArmorSubType::Robe
        );
    }

    #[test]
    fn test_soft_armor_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(SoftArmorSubType::FilthyRags), 99);
    }

    #[test]
    fn test_bracers_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(
            BracersSubType::try_from(1usize).unwrap(),
            BracersSubType::BracersOfProtection
        );
    }

    #[test]
    fn test_bracers_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(BracersSubType::PlatinumBracelet), 50);
    }

    #[test]
    fn test_belt_subtype_try_from_usize_accepts_known_value() {
        assert_eq!(BeltSubType::try_from(1usize).unwrap(), BeltSubType::Sash);
    }

    #[test]
    fn test_belt_subtype_into_usize_returns_expected_code() {
        assert_eq!(usize::from(BeltSubType::WyrmhideBelt), 8);
    }
}
