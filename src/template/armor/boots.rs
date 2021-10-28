use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BootsTemplate {
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

pub fn generate_boots(item_level: u8, template: BootsTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Boots as u8,
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

impl BootsTemplate {
    fn name(&self) -> &str {
        match self {
            BootsTemplate::SoftLeatherShoes => "Soft Leather Shoes^ [%P6,%P4]",
            BootsTemplate::SoftLeatherBoots => "Soft Leather Boots^ [%P6,%P4]",
            BootsTemplate::HardLeatherBoots => "Hard Leather Boots^ [%P6,%P4]",
            BootsTemplate::Sandals => "Sandals^ [%P6,%P4]",
            BootsTemplate::ChainBoots => "Chain Boots^ [%P6,%P4]",
            BootsTemplate::LightPlatedBoots => "Light Plated Boots^ [%P6,%P4]",
            BootsTemplate::SharkskinBoots => "Sharkskin Boots^ [%P6,%P4]",
            BootsTemplate::DemonhideBoots => "Demonhide Boots^ [%P6,%P4]",
            BootsTemplate::WyrmhideBoot => "Wyrmhide Boots^ [%P6,%P4]",
        }
    }

    fn cost(&self) -> i64 {
        match self {
            BootsTemplate::SoftLeatherShoes => 4,
            BootsTemplate::SoftLeatherBoots => 7,
            BootsTemplate::HardLeatherBoots => 12,
            BootsTemplate::Sandals => 1,
            BootsTemplate::ChainBoots => 100,
            BootsTemplate::LightPlatedBoots => 150,
            BootsTemplate::SharkskinBoots => 400,
            BootsTemplate::DemonhideBoots => 500,
            BootsTemplate::WyrmhideBoot => 1000,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            BootsTemplate::SoftLeatherShoes => 1,
            BootsTemplate::SoftLeatherBoots => 2,
            BootsTemplate::HardLeatherBoots => 3,
            BootsTemplate::Sandals => 4,
            BootsTemplate::ChainBoots => 5,
            BootsTemplate::LightPlatedBoots => 6,
            BootsTemplate::SharkskinBoots => 7,
            BootsTemplate::DemonhideBoots => 8,
            BootsTemplate::WyrmhideBoot => 9,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BootsTemplate::SoftLeatherShoes => 5,
            BootsTemplate::SoftLeatherBoots => 20,
            BootsTemplate::HardLeatherBoots => 40,
            BootsTemplate::Sandals => 1,
            BootsTemplate::ChainBoots => 60,
            BootsTemplate::LightPlatedBoots => 80,
            BootsTemplate::SharkskinBoots => 50,
            BootsTemplate::DemonhideBoots => 50,
            BootsTemplate::WyrmhideBoot => 50,
        }
    }

    fn ac(&self) -> i16 {
        match self {
            BootsTemplate::SoftLeatherShoes => 1,
            BootsTemplate::SoftLeatherBoots => 2,
            BootsTemplate::HardLeatherBoots => 3,
            BootsTemplate::Sandals => 0,
            BootsTemplate::ChainBoots => 4,
            BootsTemplate::LightPlatedBoots => 5,
            BootsTemplate::SharkskinBoots => 6,
            BootsTemplate::DemonhideBoots => 7,
            BootsTemplate::WyrmhideBoot => 8,
        }
    }
}

