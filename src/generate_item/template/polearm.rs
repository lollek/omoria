use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ItemSubType, PoleArmSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PolearmTemplate {
    AwlPike,
    BeakedAxe,
    Fauchard,
    Glaive,
    Halberd,
    LucerneHammer,
    Pike,
    Spike,
    Lance,
    Javelin,
    Naginata,
    WarScythe,
}

impl PolearmTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(PolearmTemplate::AwlPike),
            Box::new(PolearmTemplate::BeakedAxe),
            Box::new(PolearmTemplate::Fauchard),
            Box::new(PolearmTemplate::Glaive),
            Box::new(PolearmTemplate::Halberd),
            Box::new(PolearmTemplate::LucerneHammer),
            Box::new(PolearmTemplate::Pike),
            Box::new(PolearmTemplate::Spike),
            Box::new(PolearmTemplate::Lance),
            Box::new(PolearmTemplate::Javelin),
            Box::new(PolearmTemplate::Naginata),
            Box::new(PolearmTemplate::WarScythe),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        PolearmTemplate::vec().into_iter()
    }
}

impl ItemTemplate for PolearmTemplate {
    fn name(&self) -> &str {
        match self {
            PolearmTemplate::AwlPike => "Awl-Pike (%P0)^ (%P2,%P3)",
            PolearmTemplate::BeakedAxe => "Beaked Axe (%P0)^ (%P2,%P3)",
            PolearmTemplate::Fauchard => "Fauchard (%P0)^ (%P2,%P3)",
            PolearmTemplate::Glaive => "Glaive (%P0)^ (%P2,%P3)",
            PolearmTemplate::Halberd => "Halberd (%P0)^ (%P2,%P3)",
            PolearmTemplate::LucerneHammer => "Lucerne Hammer (%P0)^ (%P2,%P3)",
            PolearmTemplate::Pike => "Pike (%P0)^ (%P2,%P3)",
            PolearmTemplate::Spike => "Spear (%P0)^ (%P2,%P3)",
            PolearmTemplate::Lance => "Lance (%P0)^ (%P2,%P3)",
            PolearmTemplate::Javelin => "Javelin (%P0)^ (%P2,%P3)",
            PolearmTemplate::Naginata => "Naginata (%P0)^ (%P2,%P3)",
            PolearmTemplate::WarScythe => "War Scythe (%P0)^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::PoleArm
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0x10000000
    }
    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            PolearmTemplate::AwlPike => 340,
            PolearmTemplate::BeakedAxe => 408,
            PolearmTemplate::Fauchard => 376,
            PolearmTemplate::Glaive => 363,
            PolearmTemplate::Halberd => 430,
            PolearmTemplate::LucerneHammer => 376,
            PolearmTemplate::Pike => 358,
            PolearmTemplate::Spike => 36,
            PolearmTemplate::Lance => 230,
            PolearmTemplate::Javelin => 18,
            PolearmTemplate::Naginata => 1500,
            PolearmTemplate::WarScythe => 550,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            PolearmTemplate::AwlPike => ItemSubType::PoleArm(PoleArmSubType::AwlPike),
            PolearmTemplate::BeakedAxe => ItemSubType::PoleArm(PoleArmSubType::BeakedAxe),
            PolearmTemplate::Fauchard => ItemSubType::PoleArm(PoleArmSubType::Fauchard),
            PolearmTemplate::Glaive => ItemSubType::PoleArm(PoleArmSubType::Glaive),
            PolearmTemplate::Halberd => ItemSubType::PoleArm(PoleArmSubType::Halberd),
            PolearmTemplate::LucerneHammer => ItemSubType::PoleArm(PoleArmSubType::LucerneHammer),
            PolearmTemplate::Pike => ItemSubType::PoleArm(PoleArmSubType::Pike),
            PolearmTemplate::Spike => ItemSubType::PoleArm(PoleArmSubType::Spike),
            PolearmTemplate::Lance => ItemSubType::PoleArm(PoleArmSubType::Lance),
            PolearmTemplate::Javelin => ItemSubType::PoleArm(PoleArmSubType::Javelin),
            PolearmTemplate::Naginata => ItemSubType::PoleArm(PoleArmSubType::Naginata),
            PolearmTemplate::WarScythe => ItemSubType::PoleArm(PoleArmSubType::WarScythe),
        }
    }

    fn weight(&self) -> u16 {
        match self {
            PolearmTemplate::AwlPike => 160,
            PolearmTemplate::BeakedAxe => 180,
            PolearmTemplate::Fauchard => 170,
            PolearmTemplate::Glaive => 190,
            PolearmTemplate::Halberd => 190,
            PolearmTemplate::LucerneHammer => 120,
            PolearmTemplate::Pike => 160,
            PolearmTemplate::Spike => 50,
            PolearmTemplate::Lance => 300,
            PolearmTemplate::Javelin => 30,
            PolearmTemplate::Naginata => 250,
            PolearmTemplate::WarScythe => 210,
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
            PolearmTemplate::AwlPike => "1d8",
            PolearmTemplate::BeakedAxe => "2d6",
            PolearmTemplate::Fauchard => "1d10",
            PolearmTemplate::Glaive => "2d6",
            PolearmTemplate::Halberd => "3d3",
            PolearmTemplate::LucerneHammer => "2d5",
            PolearmTemplate::Pike => "2d5",
            PolearmTemplate::Spike => "1d6",
            PolearmTemplate::Lance => "2d8",
            PolearmTemplate::Javelin => "1d4",
            PolearmTemplate::Naginata => "5d5",
            PolearmTemplate::WarScythe => "3d5",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            PolearmTemplate::AwlPike => 8,
            PolearmTemplate::BeakedAxe => 15,
            PolearmTemplate::Fauchard => 17,
            PolearmTemplate::Glaive => 20,
            PolearmTemplate::Halberd => 22,
            PolearmTemplate::LucerneHammer => 11,
            PolearmTemplate::Pike => 15,
            PolearmTemplate::Spike => 5,
            PolearmTemplate::Lance => 10,
            PolearmTemplate::Javelin => 4,
            PolearmTemplate::Naginata => 50,
            PolearmTemplate::WarScythe => 30,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
