use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum GlovesTemplate {
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

pub fn generate_gloves(item_level: u8, template: GlovesTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Gloves as u8,
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

impl GlovesTemplate {
    fn name(&self) -> &str {
        match self {
            GlovesTemplate::LeatherGloves => "Leather Gloves^ [%P6,%P4]",
            GlovesTemplate::HeavyGloves => "Heavy Gloves^ [%P6,%P4]",
            GlovesTemplate::ClothGloves => "Cloth Gloves^ [%P6,%P4]",
            GlovesTemplate::ChainGloves => "Chain Gloves^ [%P6,%P4]",
            GlovesTemplate::LightGauntlets => "Light Gauntlets^ [%P6,%P4]",
            GlovesTemplate::HeavyGauntlets => "Heavy Gauntlets^ [%P6,%P4]",
            GlovesTemplate::SharkskinGloves => "Sharkskin Gloves^ [%P6,%P4]",
            GlovesTemplate::WarGauntlets => "War Gauntlets^ [%P6,%P4]",
            GlovesTemplate::DemonhideGloves => "Demonhide Gloves^ [%P6,%P4]",
            GlovesTemplate::WyrmhideGloves => "Wyrmhide Gloves^ [%P6,%P4]",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            GlovesTemplate::LeatherGloves => 3,
            GlovesTemplate::HeavyGloves => 35,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 100,
            GlovesTemplate::LightGauntlets => 150,
            GlovesTemplate::HeavyGauntlets => 200,
            GlovesTemplate::SharkskinGloves => 300,
            GlovesTemplate::WarGauntlets => 300,
            GlovesTemplate::DemonhideGloves => 400,
            GlovesTemplate::WyrmhideGloves => 1000,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 2,
            GlovesTemplate::ClothGloves => 5,
            GlovesTemplate::ChainGloves => 6,
            GlovesTemplate::LightGauntlets => 7,
            GlovesTemplate::HeavyGauntlets => 8,
            GlovesTemplate::SharkskinGloves => 9,
            GlovesTemplate::WarGauntlets => 10,
            GlovesTemplate::DemonhideGloves => 11,
            GlovesTemplate::WyrmhideGloves => 12,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            GlovesTemplate::LeatherGloves => 5,
            GlovesTemplate::HeavyGloves => 25,
            GlovesTemplate::ClothGloves => 1,
            GlovesTemplate::ChainGloves => 30,
            GlovesTemplate::LightGauntlets => 60,
            GlovesTemplate::HeavyGauntlets => 100,
            GlovesTemplate::SharkskinGloves => 30,
            GlovesTemplate::WarGauntlets => 150,
            GlovesTemplate::DemonhideGloves => 30,
            GlovesTemplate::WyrmhideGloves => 30,
        }
    }

    fn ac(&self) -> i16 {
        match self {
            GlovesTemplate::LeatherGloves => 1,
            GlovesTemplate::HeavyGloves => 2,
            GlovesTemplate::ClothGloves => 0,
            GlovesTemplate::ChainGloves => 4,
            GlovesTemplate::LightGauntlets => 5,
            GlovesTemplate::HeavyGauntlets => 7,
            GlovesTemplate::SharkskinGloves => 6,
            GlovesTemplate::WarGauntlets => 8,
            GlovesTemplate::DemonhideGloves => 7,
            GlovesTemplate::WyrmhideGloves => 8,
        }
    }
}
