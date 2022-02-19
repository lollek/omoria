use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LightSourceTemplate {
    WoodenTorch,
    BrassLantern,
    MagicTorch,
    MagicLantern,
}

impl LightSourceTemplate {
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
        vec![
            Box::new(LightSourceTemplate::WoodenTorch),
            Box::new(LightSourceTemplate::BrassLantern),
            Box::new(LightSourceTemplate::MagicTorch),
            Box::new(LightSourceTemplate::MagicLantern),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        LightSourceTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            13 => Box::new(LightSourceTemplate::WoodenTorch),
            1 => Box::new(LightSourceTemplate::BrassLantern),
            30 => Box::new(LightSourceTemplate::MagicTorch),
            17 => Box::new(LightSourceTemplate::MagicLantern),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for LightSourceTemplate {
    fn name(&self, item: &model::Item) -> String {
        let mut parts = Vec::new();
        parts.push(item_name::number_of(&item));
        parts.push(
            match self {
                LightSourceTemplate::WoodenTorch => Cow::from(format!("Wooden Torch{}", if item.number == 1 { "" } else { "es" })),
                LightSourceTemplate::BrassLantern => Cow::from(format!("Brass Lantern{}", if item.number == 1 { "" } else { "s" })),
                LightSourceTemplate::MagicTorch => Cow::from("Magic Torch"),
                LightSourceTemplate::MagicLantern => Cow::from("Magic Lantern"),
            });
        let turns_plural = if item.p1 == 1 { "" } else { "s" };
        parts.push(Cow::from(format!(" with {} turn{} of light", item.p1, turns_plural)));
        parts.join("")
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::LightSource }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }


    fn p1(&self) -> i64 {
        match self {
            LightSourceTemplate::WoodenTorch => 4000,
            LightSourceTemplate::BrassLantern => 7500,
            LightSourceTemplate::MagicTorch => 9000,
            LightSourceTemplate::MagicLantern => 20000,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            LightSourceTemplate::WoodenTorch => 2,
            LightSourceTemplate::BrassLantern => 35,
            LightSourceTemplate::MagicTorch => 30,
            LightSourceTemplate::MagicLantern => 200,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            LightSourceTemplate::WoodenTorch => 13,
            LightSourceTemplate::BrassLantern => 1,
            LightSourceTemplate::MagicTorch => 30,
            LightSourceTemplate::MagicLantern => 17,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            LightSourceTemplate::WoodenTorch => 30,
            LightSourceTemplate::BrassLantern => 50,
            LightSourceTemplate::MagicTorch => 1,
            LightSourceTemplate::MagicLantern => 45,
        }
    }

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

    fn damage(&self) -> &str {
        match self {
            LightSourceTemplate::WoodenTorch => "1d4",
            LightSourceTemplate::BrassLantern => "1d3",
            LightSourceTemplate::MagicTorch => "2d6",
            LightSourceTemplate::MagicLantern => "1d8",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            LightSourceTemplate::WoodenTorch => 1,
            LightSourceTemplate::BrassLantern => 20,
            LightSourceTemplate::MagicTorch => 10,
            LightSourceTemplate::MagicLantern => 1,
        }
    }

    fn is_identified(&self) -> bool { false }
}
