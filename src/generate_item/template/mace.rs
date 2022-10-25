use super::super::item_template::ItemTemplate;
use crate::model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MaceTemplate {
    BallAndChain,
    WoodenClub,
    Flail,
    GreatFlail,
    MorningStar,
    Mace,
    WarHammer,
    LeadFilledMace,
    IronShodQuarterstaff,
    OgreMaul,
}

impl MaceTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(MaceTemplate::BallAndChain),
            Box::new(MaceTemplate::WoodenClub),
            Box::new(MaceTemplate::Flail),
            Box::new(MaceTemplate::GreatFlail),
            Box::new(MaceTemplate::MorningStar),
            Box::new(MaceTemplate::Mace),
            Box::new(MaceTemplate::WarHammer),
            Box::new(MaceTemplate::LeadFilledMace),
            Box::new(MaceTemplate::IronShodQuarterstaff),
            Box::new(MaceTemplate::OgreMaul),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        MaceTemplate::vec().into_iter()
    }
}

impl ItemTemplate for MaceTemplate {
    fn name(&self) -> &str {
        match self {
            MaceTemplate::BallAndChain => "Ball and Chain (%P0)^ (%P2,%P3)",
            MaceTemplate::WoodenClub => "Wooden Club (%P0)^ (%P2,%P3)",
            MaceTemplate::Flail => "Flail (%P0)^ (%P2,%P3)",
            MaceTemplate::GreatFlail => "Two Handed Great Flail (%P0)^ (%P2,%P3)",
            MaceTemplate::MorningStar => "Morningstar (%P0)^ (%P2,%P3)",
            MaceTemplate::Mace => "Mace (%P0)^ (%P2,%P3)",
            MaceTemplate::WarHammer => "War Hammer (%P0)^ (%P2,%P3)",
            MaceTemplate::LeadFilledMace => "Lead Filled Mace (%P0)^ (%P2,%P3)",
            MaceTemplate::IronShodQuarterstaff => "Iron Shod Quarterstaff (%P0)^ (%P2,%P3)",
            MaceTemplate::OgreMaul => "Ogre Maul (%P0)^ (%P2,%P3)",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Maul
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
            MaceTemplate::BallAndChain => 200,
            MaceTemplate::WoodenClub => 1,
            MaceTemplate::Flail => 353,
            MaceTemplate::GreatFlail => 590,
            MaceTemplate::MorningStar => 396,
            MaceTemplate::Mace => 130,
            MaceTemplate::WarHammer => 225,
            MaceTemplate::LeadFilledMace => 502,
            MaceTemplate::IronShodQuarterstaff => 25,
            MaceTemplate::OgreMaul => 1050,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            MaceTemplate::BallAndChain => 2,
            MaceTemplate::WoodenClub => 6,
            MaceTemplate::Flail => 7,
            MaceTemplate::GreatFlail => 8,
            MaceTemplate::MorningStar => 9,
            MaceTemplate::Mace => 10,
            MaceTemplate::WarHammer => 11,
            MaceTemplate::LeadFilledMace => 12,
            MaceTemplate::IronShodQuarterstaff => 13,
            MaceTemplate::OgreMaul => 14,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            MaceTemplate::BallAndChain => 150,
            MaceTemplate::WoodenClub => 100,
            MaceTemplate::Flail => 150,
            MaceTemplate::GreatFlail => 280,
            MaceTemplate::MorningStar => 150,
            MaceTemplate::Mace => 120,
            MaceTemplate::WarHammer => 120,
            MaceTemplate::LeadFilledMace => 180,
            MaceTemplate::IronShodQuarterstaff => 100,
            MaceTemplate::OgreMaul => 350,
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
            MaceTemplate::BallAndChain => "2d4",
            MaceTemplate::WoodenClub => "1d3",
            MaceTemplate::Flail => "2d6",
            MaceTemplate::GreatFlail => "3d6",
            MaceTemplate::MorningStar => "2d6",
            MaceTemplate::Mace => "2d4",
            MaceTemplate::WarHammer => "3d3",
            MaceTemplate::LeadFilledMace => "3d4",
            MaceTemplate::IronShodQuarterstaff => "1d5",
            MaceTemplate::OgreMaul => "3d9",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            MaceTemplate::BallAndChain => 20,
            MaceTemplate::WoodenClub => 0,
            MaceTemplate::Flail => 12,
            MaceTemplate::GreatFlail => 45,
            MaceTemplate::MorningStar => 10,
            MaceTemplate::Mace => 6,
            MaceTemplate::WarHammer => 5,
            MaceTemplate::LeadFilledMace => 15,
            MaceTemplate::IronShodQuarterstaff => 2,
            MaceTemplate::OgreMaul => 50,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
