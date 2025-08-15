use super::super::item_template::ItemTemplate;
use crate::generate_item::item_template::create_melee_weapon;
use crate::generate_item::ItemQuality;
use crate::misc::{rs2item_damage, rs2item_name};
use crate::model::WornFlag1::{
    GivesCharisma, Regeneration, ResistAcid, ResistCold, Searching, Stealth,
};
use crate::model::{
    self,
    item_subtype::{ItemSubType, MaulSubType},
    Item,
};

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

    pub fn apply_club_of_trollkind(item: &mut Item) {
        item.name = rs2item_name("Club of Trollkind");
        item.apply_wornflag1(GivesCharisma); // Should give negative charisma
        item.apply_wornflag1(Searching); // Should give negative searching
        item.apply_wornflag1(Stealth); // Should give negative stealth
        item.apply_wornflag1(Regeneration);
        item.apply_wornflag1(ResistAcid);
        item.apply_wornflag1(ResistCold);
        item.p1 = -5;
        item.cost = 120_000;
        item.damage = rs2item_damage("3d4");
    }
}

impl ItemTemplate for MaceTemplate {
    fn create(&self, item_quality: ItemQuality) -> Item {
        create_melee_weapon(self, item_quality)
    }

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

    fn subtype(&self) -> ItemSubType {
        match self {
            MaceTemplate::BallAndChain => ItemSubType::Maul(MaulSubType::BallAndChain),
            MaceTemplate::WoodenClub => ItemSubType::Maul(MaulSubType::WoodenClub),
            MaceTemplate::Flail => ItemSubType::Maul(MaulSubType::Flail),
            MaceTemplate::GreatFlail => ItemSubType::Maul(MaulSubType::GreatFlail),
            MaceTemplate::MorningStar => ItemSubType::Maul(MaulSubType::MorningStar),
            MaceTemplate::Mace => ItemSubType::Maul(MaulSubType::Mace),
            MaceTemplate::WarHammer => ItemSubType::Maul(MaulSubType::WarHammer),
            MaceTemplate::LeadFilledMace => ItemSubType::Maul(MaulSubType::LeadFilledMace),
            MaceTemplate::IronShodQuarterstaff => {
                ItemSubType::Maul(MaulSubType::IronShodQuarterstaff)
            }
            MaceTemplate::OgreMaul => ItemSubType::Maul(MaulSubType::OgreMaul),
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
