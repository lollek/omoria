use misc;
use model;

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

pub fn generate_mace(item_level: u8, template: MaceTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: model::ItemType::Maul as u8,
        flags: 0,
        flags2: 0,
        p1: 0,
        cost: template.cost(),
        subval: template.subval(),
        weight: template.weight(),
        number: 1,
        tohit: 0,
        todam: 0,
        ac: 0,
        toac: 0,
        damage: misc::rs2item_damage(template.damage()),
        level: item_level as i8,
        identified: 0,
    }
}

impl MaceTemplate {
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

    fn subval(&self) -> i64 {
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
}
