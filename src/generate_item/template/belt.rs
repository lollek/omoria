use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{BeltSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BeltTemplate {
    Sash,
    LightBelt,
    Belt,
    HeavyBelt,
    LightPlatedBelt,
    SharkskinBelt,
    DemonhideBelt,
    WyrmhideBelt,
}

impl BeltTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(BeltTemplate::Sash),
            Box::new(BeltTemplate::LightBelt),
            Box::new(BeltTemplate::Belt),
            Box::new(BeltTemplate::HeavyBelt),
            Box::new(BeltTemplate::LightPlatedBelt),
            Box::new(BeltTemplate::SharkskinBelt),
            Box::new(BeltTemplate::DemonhideBelt),
            Box::new(BeltTemplate::WyrmhideBelt),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        BeltTemplate::vec().into_iter()
    }
}

impl ItemTemplate for BeltTemplate {
    fn name(&self) -> &str {
        match self {
            BeltTemplate::Sash => "Sash^ [%P6,%P4]",
            BeltTemplate::LightBelt => "Light Belt^ [%P6,%P4]",
            BeltTemplate::Belt => "Belt^ [%P6,%P4]",
            BeltTemplate::HeavyBelt => "Heavy Belt^ [%P6,%P4]",
            BeltTemplate::LightPlatedBelt => "Light Plated Belt^ [%P6,%P4]",
            BeltTemplate::SharkskinBelt => "Sharkskin Belt^ [%P6,%P4]",
            BeltTemplate::DemonhideBelt => "Demonhide Belt^ [%P6,%P4]",
            BeltTemplate::WyrmhideBelt => "Wyrmhide Belt^ [%P6,%P4]",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Belt
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
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 5,
            BeltTemplate::Belt => 7,
            BeltTemplate::HeavyBelt => 20,
            BeltTemplate::LightPlatedBelt => 50,
            BeltTemplate::SharkskinBelt => 200,
            BeltTemplate::DemonhideBelt => 300,
            BeltTemplate::WyrmhideBelt => 500,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            BeltTemplate::Sash => ItemSubType::Belt(BeltSubType::Sash),
            BeltTemplate::LightBelt => ItemSubType::Belt(BeltSubType::LightBelt),
            BeltTemplate::Belt => ItemSubType::Belt(BeltSubType::Belt),
            BeltTemplate::HeavyBelt => ItemSubType::Belt(BeltSubType::HeavyBelt),
            BeltTemplate::LightPlatedBelt => ItemSubType::Belt(BeltSubType::LightPlatedBelt),
            BeltTemplate::SharkskinBelt => ItemSubType::Belt(BeltSubType::SharkskinBelt),
            BeltTemplate::DemonhideBelt => ItemSubType::Belt(BeltSubType::DemonhideBelt),
            BeltTemplate::WyrmhideBelt => ItemSubType::Belt(BeltSubType::WyrmhideBelt),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BeltTemplate::Sash => 1,
            BeltTemplate::LightBelt => 5,
            BeltTemplate::Belt => 10,
            BeltTemplate::HeavyBelt => 15,
            BeltTemplate::LightPlatedBelt => 30,
            BeltTemplate::SharkskinBelt => 10,
            BeltTemplate::DemonhideBelt => 10,
            BeltTemplate::WyrmhideBelt => 10,
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
            BeltTemplate::Sash => 0,
            BeltTemplate::LightBelt => 0,
            BeltTemplate::Belt => 0,
            BeltTemplate::HeavyBelt => 1,
            BeltTemplate::LightPlatedBelt => 2,
            BeltTemplate::SharkskinBelt => 3,
            BeltTemplate::DemonhideBelt => 4,
            BeltTemplate::WyrmhideBelt => 5,
        }
    }

    fn modifier_to_ac(&self) -> i16 {
        0
    }
    fn damage(&self) -> &str {
        "1d1"
    }

    fn item_level(&self) -> u8 {
        match self {
            BeltTemplate::Sash => 0,
            BeltTemplate::LightBelt => 0,
            BeltTemplate::Belt => 0,
            BeltTemplate::HeavyBelt => 6,
            BeltTemplate::LightPlatedBelt => 15,
            BeltTemplate::SharkskinBelt => 30,
            BeltTemplate::DemonhideBelt => 40,
            BeltTemplate::WyrmhideBelt => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
