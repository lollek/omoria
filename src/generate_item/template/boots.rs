use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{BootsSubType, ItemSubType},
};

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

impl BootsTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(BootsTemplate::SoftLeatherShoes),
            Box::new(BootsTemplate::SoftLeatherBoots),
            Box::new(BootsTemplate::HardLeatherBoots),
            Box::new(BootsTemplate::Sandals),
            Box::new(BootsTemplate::ChainBoots),
            Box::new(BootsTemplate::LightPlatedBoots),
            Box::new(BootsTemplate::SharkskinBoots),
            Box::new(BootsTemplate::DemonhideBoots),
            Box::new(BootsTemplate::WyrmhideBoot),
        ]
    }
    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        BootsTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BootsTemplate {
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

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Boots
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0
    }
    fn p1(&self) -> i64 {
        0
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

    fn subtype(&self) -> ItemSubType {
        match self {
            BootsTemplate::SoftLeatherShoes => ItemSubType::Boots(BootsSubType::SoftLeatherShoes),
            BootsTemplate::SoftLeatherBoots => ItemSubType::Boots(BootsSubType::SoftLeatherBoots),
            BootsTemplate::HardLeatherBoots => ItemSubType::Boots(BootsSubType::HardLeatherBoots),
            BootsTemplate::Sandals => ItemSubType::Boots(BootsSubType::Sandals),
            BootsTemplate::ChainBoots => ItemSubType::Boots(BootsSubType::ChainBoots),
            BootsTemplate::LightPlatedBoots => ItemSubType::Boots(BootsSubType::LightPlatedBoots),
            BootsTemplate::SharkskinBoots => ItemSubType::Boots(BootsSubType::SharkskinBoots),
            BootsTemplate::DemonhideBoots => ItemSubType::Boots(BootsSubType::DemonhideBoots),
            BootsTemplate::WyrmhideBoot => ItemSubType::Boots(BootsSubType::WyrmhideBoot),
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

    fn number(&self) -> u16 {
        1
    }
    fn modifier_to_hit(&self) -> i16 {
        0
    }
    fn modifier_to_damage(&self) -> i16 {
        0
    }

    fn base_ac(&self) -> i16 {
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

    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "0d0"
    }

    fn item_level(&self) -> u8 {
        match self {
            BootsTemplate::SoftLeatherShoes => 1,
            BootsTemplate::SoftLeatherBoots => 4,
            BootsTemplate::HardLeatherBoots => 6,
            BootsTemplate::Sandals => 1,
            BootsTemplate::ChainBoots => 10,
            BootsTemplate::LightPlatedBoots => 16,
            BootsTemplate::SharkskinBoots => 30,
            BootsTemplate::DemonhideBoots => 40,
            BootsTemplate::WyrmhideBoot => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
