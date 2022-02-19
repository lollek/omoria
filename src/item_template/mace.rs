use std::borrow::Cow;

use model;
use item_template;
use logic::item_name;

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
    pub fn vec() -> Vec<Box<dyn item_template::ItemTemplate>> {
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

    pub fn iter() -> impl Iterator<Item=Box<dyn item_template::ItemTemplate>> {
        MaceTemplate::vec().into_iter()
    }

    pub fn from(subval: i64) -> Box<dyn item_template::ItemTemplate> {
        match subval {
            2 => Box::new(MaceTemplate::BallAndChain),
            6 => Box::new(MaceTemplate::WoodenClub),
            7 => Box::new(MaceTemplate::Flail),
            8 => Box::new(MaceTemplate::GreatFlail),
            9 => Box::new(MaceTemplate::MorningStar),
            10 => Box::new(MaceTemplate::Mace),
            11 => Box::new(MaceTemplate::WarHammer),
            12 => Box::new(MaceTemplate::LeadFilledMace),
            13 => Box::new(MaceTemplate::IronShodQuarterstaff),
            14 => Box::new(MaceTemplate::OgreMaul),
            _ => panic!("subval {} out of bounds", subval),
        }
    }
}

impl item_template::ItemTemplate for MaceTemplate {

    fn name(&self, item: &model::Item) -> String {
        item_name::generate_weapon_name(item,
            Cow::from(match self {
                MaceTemplate::BallAndChain => "Ball and Chain",
                MaceTemplate::WoodenClub => "Wooden Club",
                MaceTemplate::Flail => "Flail",
                MaceTemplate::GreatFlail => "Two Handed Great Flail",
                MaceTemplate::MorningStar => "Morningstar",
                MaceTemplate::Mace => "Mace",
                MaceTemplate::WarHammer => "War Hammer",
                MaceTemplate::LeadFilledMace => "Lead Filled Mace",
                MaceTemplate::IronShodQuarterstaff => "Iron Shod Quarterstaff",
                MaceTemplate::OgreMaul => "Ogre Maul",
            }))
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Mace }
    fn flags1(&self) -> u64 { 0 }
    fn flags2(&self) -> u64 { 0 }
    fn p1(&self) -> i64 { 0 }

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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }

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

    fn is_identified(&self) -> bool { false }
}
