use model;
use item_template;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        BootsTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            1 => Box::new(BootsTemplate::SoftLeatherShoes),
            2 => Box::new(BootsTemplate::SoftLeatherBoots),
            3 => Box::new(BootsTemplate::HardLeatherBoots),
            4 => Box::new(BootsTemplate::Sandals),
            5 => Box::new(BootsTemplate::ChainBoots),
            6 => Box::new(BootsTemplate::LightPlatedBoots),
            7 => Box::new(BootsTemplate::SharkskinBoots),
            8 => Box::new(BootsTemplate::DemonhideBoots),
            9 => Box::new(BootsTemplate::WyrmhideBoot),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for BootsTemplate {
    fn item_type(&self) -> model::ItemType { model::ItemType::Boots }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

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

    fn subtype(&self) -> i64 {
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }

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

    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "0d0" }

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

    fn is_identified(&self) -> bool { false }

}

