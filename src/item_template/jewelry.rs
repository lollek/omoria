use model;
use item_template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum JewelryTemplate {
    SmallGoldPendant,
    SmallMithrilPendant,
    LargeMithrilGarterBelt,
    SmallSilverPendant,
}

impl JewelryTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(JewelryTemplate::SmallGoldPendant),
            Box::new(JewelryTemplate::SmallMithrilPendant),
            Box::new(JewelryTemplate::LargeMithrilGarterBelt),
            Box::new(JewelryTemplate::SmallSilverPendant),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        JewelryTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            274 => Box::new(JewelryTemplate::SmallGoldPendant),
            275 => Box::new(JewelryTemplate::SmallMithrilPendant),
            276 => Box::new(JewelryTemplate::LargeMithrilGarterBelt),
            266 => Box::new(JewelryTemplate::SmallSilverPendant),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for JewelryTemplate {
    fn item_type(&self) -> model::ItemType {
        match self {
            JewelryTemplate::SmallGoldPendant => model::ItemType::Jewelry,
            JewelryTemplate::SmallMithrilPendant => model::ItemType::Jewelry,
            JewelryTemplate::LargeMithrilGarterBelt => model::ItemType::Jewelry,
            JewelryTemplate::SmallSilverPendant => model::ItemType::Jewelry,
        }
    }

    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

    fn cost(&self) -> i64 {
        match self {
            JewelryTemplate::SmallGoldPendant => 75,
            JewelryTemplate::SmallMithrilPendant => 350,
            JewelryTemplate::LargeMithrilGarterBelt => 1500,
            JewelryTemplate::SmallSilverPendant => 60,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            JewelryTemplate::SmallGoldPendant => 274,
            JewelryTemplate::SmallMithrilPendant => 275,
            JewelryTemplate::LargeMithrilGarterBelt => 276,
            JewelryTemplate::SmallSilverPendant => 266,
        }
    }

    fn weight(&self) -> u16 { 5 }
    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            JewelryTemplate::SmallGoldPendant => 5,
            JewelryTemplate::SmallMithrilPendant => 10,
            JewelryTemplate::LargeMithrilGarterBelt => 45,
            JewelryTemplate::SmallSilverPendant => 5,
        }
    }

    fn is_identified(&self) -> bool { false }
}

