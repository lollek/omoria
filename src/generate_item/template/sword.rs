use crate::generate_item::item_template::WornFlag2;
use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ItemSubType, SwordSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SwordTemplate {
    Backsword,
    BastardSword,
    Broadsword,
    Claymore,
    Cutlass,
    Espadon,
    ExecutionersSword,
    Flamberge,
    Katana,
    Longsword,
    Nodachi,
    Sabre,
    Zweihander,
    BrokenSword,
}

impl SwordTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(SwordTemplate::Backsword),
            Box::new(SwordTemplate::BastardSword),
            Box::new(SwordTemplate::Broadsword),
            Box::new(SwordTemplate::Claymore),
            Box::new(SwordTemplate::Cutlass),
            Box::new(SwordTemplate::Espadon),
            Box::new(SwordTemplate::ExecutionersSword),
            Box::new(SwordTemplate::Flamberge),
            Box::new(SwordTemplate::Katana),
            Box::new(SwordTemplate::Longsword),
            Box::new(SwordTemplate::Nodachi),
            Box::new(SwordTemplate::Sabre),
            Box::new(SwordTemplate::Zweihander),
            Box::new(SwordTemplate::BrokenSword),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        SwordTemplate::vec().into_iter()
    }
}

impl ItemTemplate for SwordTemplate {
    fn name(&self) -> &str {
        match self {
            SwordTemplate::Backsword => "Backsword (%P0)^ (%P2,%P3)",
            SwordTemplate::BastardSword => "Bastard Sword (%P0)^ (%P2,%P3)",
            SwordTemplate::Broadsword => "Broadsword (%P0)^ (%P2,%P3)",
            SwordTemplate::Claymore => "Claymore (%P0)^ (%P2,%P3)",
            SwordTemplate::Cutlass => "Cutlass (%P0)^ (%P2,%P3)",
            SwordTemplate::Espadon => "Espadon (%P0)^ (%P2,%P3)",
            SwordTemplate::ExecutionersSword => "Executioner's Sword (%P0)^ (%P2,%P3)",
            SwordTemplate::Flamberge => "Flamberge (%P0)^ (%P2,%P3)",
            SwordTemplate::Katana => "Katana (%P0)^ (%P2,%P3)",
            SwordTemplate::Longsword => "Longsword (%P0)^ (%P2,%P3)",
            SwordTemplate::Nodachi => "No-Dachi (%P0)^ (%P2,%P3)",
            SwordTemplate::Sabre => "Sabre (%P0)^ (%P2,%P3)",
            SwordTemplate::Zweihander => "Zweihander (%P0)^ (%P2,%P3)",
            SwordTemplate::BrokenSword => "Broken Sword (%P0)^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Sword
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        WornFlag2::Sharp as u64
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            SwordTemplate::Backsword => 60,
            SwordTemplate::BastardSword => 225,
            SwordTemplate::Broadsword => 775,
            SwordTemplate::Claymore => 85,
            SwordTemplate::Cutlass => 655,
            SwordTemplate::Espadon => 850,
            SwordTemplate::ExecutionersSword => 1000,
            SwordTemplate::Flamberge => 400,
            SwordTemplate::Katana => 300,
            SwordTemplate::Longsword => 675,
            SwordTemplate::Nodachi => 50,
            SwordTemplate::Sabre => 1000,
            SwordTemplate::Zweihander => 0,
            SwordTemplate::BrokenSword => 0,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            SwordTemplate::Backsword => ItemSubType::Sword(SwordSubType::Backsword),
            SwordTemplate::BastardSword => ItemSubType::Sword(SwordSubType::BastardSword),
            SwordTemplate::Broadsword => ItemSubType::Sword(SwordSubType::Broadsword),
            SwordTemplate::Claymore => ItemSubType::Sword(SwordSubType::Claymore),
            SwordTemplate::Cutlass => ItemSubType::Sword(SwordSubType::Cutlass),
            SwordTemplate::Espadon => ItemSubType::Sword(SwordSubType::Espadon),
            SwordTemplate::ExecutionersSword => ItemSubType::Sword(SwordSubType::ExecutionersSword),
            SwordTemplate::Flamberge => ItemSubType::Sword(SwordSubType::Flamberge),
            SwordTemplate::Katana => ItemSubType::Sword(SwordSubType::Katana),
            SwordTemplate::Longsword => ItemSubType::Sword(SwordSubType::Longsword),
            SwordTemplate::Nodachi => ItemSubType::Sword(SwordSubType::Nodachi),
            SwordTemplate::Sabre => ItemSubType::Sword(SwordSubType::Sabre),
            SwordTemplate::Zweihander => ItemSubType::Sword(SwordSubType::Zweihander),
            SwordTemplate::BrokenSword => ItemSubType::Sword(SwordSubType::BrokenSword),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            SwordTemplate::Backsword => 95,
            SwordTemplate::BastardSword => 140,
            SwordTemplate::Broadsword => 150,
            SwordTemplate::Claymore => 200,
            SwordTemplate::Cutlass => 110,
            SwordTemplate::Espadon => 180,
            SwordTemplate::ExecutionersSword => 260,
            SwordTemplate::Flamberge => 240,
            SwordTemplate::Katana => 120,
            SwordTemplate::Longsword => 130,
            SwordTemplate::Nodachi => 200,
            SwordTemplate::Sabre => 50,
            SwordTemplate::Zweihander => 280,
            SwordTemplate::BrokenSword => 75,
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
        0
    }
    fn modifier_to_ac(&self) -> i16 {
        0
    }

    fn damage(&self) -> &str {
        match self {
            SwordTemplate::Backsword => "1d9",
            SwordTemplate::BastardSword => "3d4",
            SwordTemplate::Broadsword => "2d5",
            SwordTemplate::Claymore => "3d6",
            SwordTemplate::Cutlass => "1d7",
            SwordTemplate::Espadon => "3d6",
            SwordTemplate::ExecutionersSword => "4d5",
            SwordTemplate::Flamberge => "4d5",
            SwordTemplate::Katana => "3d4",
            SwordTemplate::Longsword => "1d10",
            SwordTemplate::Nodachi => "4d4",
            SwordTemplate::Sabre => "1d7",
            SwordTemplate::Zweihander => "4d6",
            SwordTemplate::BrokenSword => "1d4",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            SwordTemplate::Backsword => 7,
            SwordTemplate::BastardSword => 14,
            SwordTemplate::Broadsword => 9,
            SwordTemplate::Claymore => 30,
            SwordTemplate::Cutlass => 7,
            SwordTemplate::Espadon => 35,
            SwordTemplate::ExecutionersSword => 40,
            SwordTemplate::Flamberge => 45,
            SwordTemplate::Katana => 18,
            SwordTemplate::Longsword => 12,
            SwordTemplate::Nodachi => 45,
            SwordTemplate::Sabre => 5,
            SwordTemplate::Zweihander => 50,
            SwordTemplate::BrokenSword => 0,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
