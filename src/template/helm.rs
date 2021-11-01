use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum HelmTemplate {
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

impl HelmTemplate {
    pub fn iter() -> impl Iterator<Item=HelmTemplate> {
        [
            HelmTemplate::ClothHat,
            HelmTemplate::SoftLeatherCap,
            HelmTemplate::HardLeatherCap,
            HelmTemplate::MetalCap,
            HelmTemplate::FullHelm,
            HelmTemplate::GreatHelm,
            HelmTemplate::WingedHelm,
            HelmTemplate::SilverCrown,
            HelmTemplate::SilverMask,
            HelmTemplate::GoldenCrown,
            HelmTemplate::GoldenMask,
            HelmTemplate::JewelEncrustedCrown,
        ].iter().copied()
    }
}

impl template::Template for HelmTemplate {
    fn name(&self) -> &str {
        match self {
            HelmTemplate::ClothHat => "Cloth Hat^ [%P6,%P4]",
            HelmTemplate::SoftLeatherCap => "Soft Leather Cap^ [%P6,%P4]",
            HelmTemplate::HardLeatherCap => "Hard Leather Cap^ [%P6,%P4]",
            HelmTemplate::MetalCap => "Metal Cap^ [%P6,%P4]",
            HelmTemplate::FullHelm => "Full Helm^ [%P6,%P4]",
            HelmTemplate::GreatHelm => "Great Helm^ [%P6,%P4]",
            HelmTemplate::WingedHelm => "Winged Helm^ [%P6,%P4]",
            HelmTemplate::SilverCrown => "Silver Crown^ [%P6,%P4] (%P1)",
            HelmTemplate::SilverMask => "Silver Mask^ [%P6,%P4] (%P1)",
            HelmTemplate::GoldenCrown => "Golden Crown^ [%P6,%P4] (%P1)",
            HelmTemplate::GoldenMask => "Golden Mask^ [%P6,%P4] (%P1)",
            HelmTemplate::JewelEncrustedCrown => "Jewel Encrusted Crown^ [%P6,%P4] (%P1)",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Helm }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            HelmTemplate::ClothHat => 5,
            HelmTemplate::SoftLeatherCap => 10,
            HelmTemplate::HardLeatherCap => 15,
            HelmTemplate::MetalCap => 30,
            HelmTemplate::FullHelm => 75,
            HelmTemplate::GreatHelm => 200,
            HelmTemplate::WingedHelm => 300,
            HelmTemplate::SilverCrown => 250,
            HelmTemplate::SilverMask => 350,
            HelmTemplate::GoldenCrown => 500,
            HelmTemplate::GoldenMask => 1000,
            HelmTemplate::JewelEncrustedCrown => 1000,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            HelmTemplate::ClothHat => 12,
            HelmTemplate::SoftLeatherCap => 13,
            HelmTemplate::HardLeatherCap => 14,
            HelmTemplate::MetalCap => 15,
            HelmTemplate::FullHelm => 16,
            HelmTemplate::GreatHelm => 17,
            HelmTemplate::WingedHelm => 18,
            HelmTemplate::SilverCrown => 19,
            HelmTemplate::SilverMask => 20,
            HelmTemplate::GoldenCrown => 21,
            HelmTemplate::GoldenMask => 22,
            HelmTemplate::JewelEncrustedCrown => 23,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            HelmTemplate::ClothHat => 5,
            HelmTemplate::SoftLeatherCap => 10,
            HelmTemplate::HardLeatherCap => 20,
            HelmTemplate::MetalCap => 30,
            HelmTemplate::FullHelm => 75,
            HelmTemplate::GreatHelm => 80,
            HelmTemplate::WingedHelm => 80,
            HelmTemplate::SilverCrown => 30,
            HelmTemplate::SilverMask => 40,
            HelmTemplate::GoldenCrown => 30,
            HelmTemplate::GoldenMask => 50,
            HelmTemplate::JewelEncrustedCrown => 50,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }

    fn base_ac(&self) -> i16 {
        match self {
            HelmTemplate::ClothHat => 1,
            HelmTemplate::SoftLeatherCap => 2,
            HelmTemplate::HardLeatherCap => 3,
            HelmTemplate::MetalCap => 4,
            HelmTemplate::FullHelm => 6,
            HelmTemplate::GreatHelm => 7,
            HelmTemplate::WingedHelm => 8,
            HelmTemplate::SilverCrown => 0,
            HelmTemplate::SilverMask => 1,
            HelmTemplate::GoldenCrown => 0,
            HelmTemplate::GoldenMask => 2,
            HelmTemplate::JewelEncrustedCrown => 0,
        }
    }

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "0d0" }

    fn item_level(&self) -> u8 {
        match self {
            HelmTemplate::ClothHat => 1,
            HelmTemplate::SoftLeatherCap => 2,
            HelmTemplate::HardLeatherCap => 5,
            HelmTemplate::MetalCap => 10,
            HelmTemplate::FullHelm => 20,
            HelmTemplate::GreatHelm => 30,
            HelmTemplate::WingedHelm => 50,
            HelmTemplate::SilverCrown => 20,
            HelmTemplate::SilverMask => 30,
            HelmTemplate::GoldenCrown => 40,
            HelmTemplate::GoldenMask => 50,
            HelmTemplate::JewelEncrustedCrown => 75,
        }
    }

    fn is_identified(&self) -> bool { false }
}
