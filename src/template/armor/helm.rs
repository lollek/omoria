use misc;
use model;

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

pub fn generate_helm(item_level: u8, template: HelmTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Helm as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: template.cost(),
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: template.ac(),
        toac: 0,
        damage: misc::rs2item_damage("0d0"),
        level: item_level as i8,
        identified: 0,
    }
}

impl HelmTemplate {
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

    fn subval(&self) -> i64 {
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

    fn ac(&self) -> i16 {
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
}
