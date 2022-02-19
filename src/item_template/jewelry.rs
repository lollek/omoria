use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

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
    fn name(&self, item: &model::Item) -> String {
        let plural_s = || if item.number == 1 { "" } else { "s" };

        let mut parts = Vec::new();
        parts.push(item_name::number_of(item));
        parts.push(Cow::from(
                match self {
                    JewelryTemplate::SmallGoldPendant => format!("Small gold pendant{}", plural_s()),
                    JewelryTemplate::SmallMithrilPendant => format!("Small mithril pendant{}", plural_s()),
                    JewelryTemplate::LargeMithrilGarterBelt => format!("Large mithril garter-belt{}", plural_s()),
                    JewelryTemplate::SmallSilverPendant => format!("Small silver pendant{}", plural_s()),
                }));
        parts.join("")
    }

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

