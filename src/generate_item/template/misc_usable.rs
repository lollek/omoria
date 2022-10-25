use super::super::item_template::ItemTemplate;
use crate::model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum MiscUsableTemplate {
    FlaskOfOil,
    IronSpike,
    Statue,
    SilverCross,
    GoldCross,
    MithrilCross,
    Cross,
    CorkedBottle,
}

impl MiscUsableTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(MiscUsableTemplate::FlaskOfOil),
            Box::new(MiscUsableTemplate::IronSpike),
            Box::new(MiscUsableTemplate::Statue),
            Box::new(MiscUsableTemplate::SilverCross),
            Box::new(MiscUsableTemplate::GoldCross),
            Box::new(MiscUsableTemplate::MithrilCross),
            Box::new(MiscUsableTemplate::Cross),
            Box::new(MiscUsableTemplate::CorkedBottle),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        MiscUsableTemplate::vec().into_iter()
    }
}

impl ItemTemplate for MiscUsableTemplate {
    fn name(&self) -> &str {
        match self {
            MiscUsableTemplate::FlaskOfOil => "& Flask~ of Oil",
            MiscUsableTemplate::IronSpike => "& Iron Spike~",
            MiscUsableTemplate::Statue => "& %A Statue^",
            MiscUsableTemplate::SilverCross => "& Silver Cross^",
            MiscUsableTemplate::GoldCross => "& Gold Cross^",
            MiscUsableTemplate::MithrilCross => "& Mithril Cross^",
            MiscUsableTemplate::Cross => "& %M Cross^",
            MiscUsableTemplate::CorkedBottle => "& Corked Bottle^",
        }
    }

    fn item_type(&self) -> model::ItemType {
        match self {
            MiscUsableTemplate::FlaskOfOil => model::ItemType::FlaskOfOil,
            MiscUsableTemplate::IronSpike => model::ItemType::Spike,
            MiscUsableTemplate::Statue => model::ItemType::MiscUsable,
            MiscUsableTemplate::SilverCross => model::ItemType::MiscUsable,
            MiscUsableTemplate::GoldCross => model::ItemType::MiscUsable,
            MiscUsableTemplate::MithrilCross => model::ItemType::MiscUsable,
            MiscUsableTemplate::Cross => model::ItemType::MiscUsable,
            MiscUsableTemplate::CorkedBottle => model::ItemType::MiscUsable,
        }
    }

    fn flags1(&self) -> u64 {
        0
    }

    fn flags2(&self) -> u64 {
        match self {
            MiscUsableTemplate::FlaskOfOil => 0x00040000,
            MiscUsableTemplate::IronSpike => 0,
            MiscUsableTemplate::Statue => 0,
            MiscUsableTemplate::SilverCross => 0,
            MiscUsableTemplate::GoldCross => 0,
            MiscUsableTemplate::MithrilCross => 0,
            MiscUsableTemplate::Cross => 0,
            MiscUsableTemplate::CorkedBottle => 0,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            MiscUsableTemplate::FlaskOfOil => 7500,
            MiscUsableTemplate::IronSpike => 0,
            MiscUsableTemplate::Statue => 0,
            MiscUsableTemplate::SilverCross => 0,
            MiscUsableTemplate::GoldCross => 0,
            MiscUsableTemplate::MithrilCross => 0,
            MiscUsableTemplate::Cross => 0,
            MiscUsableTemplate::CorkedBottle => 0,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            MiscUsableTemplate::FlaskOfOil => 3,
            MiscUsableTemplate::IronSpike => 1,
            MiscUsableTemplate::Statue => 20,
            MiscUsableTemplate::SilverCross => 250,
            MiscUsableTemplate::GoldCross => 500,
            MiscUsableTemplate::MithrilCross => 750,
            MiscUsableTemplate::Cross => 20,
            MiscUsableTemplate::CorkedBottle => 0,
        }
    }

    fn subtype(&self) -> i64 {
        match self {
            MiscUsableTemplate::FlaskOfOil => 257,
            MiscUsableTemplate::IronSpike => 1,
            MiscUsableTemplate::Statue => 14,
            MiscUsableTemplate::SilverCross => 16,
            MiscUsableTemplate::GoldCross => 17,
            MiscUsableTemplate::MithrilCross => 18,
            MiscUsableTemplate::Cross => 19,
            MiscUsableTemplate::CorkedBottle => 21,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            MiscUsableTemplate::FlaskOfOil => 10,
            MiscUsableTemplate::IronSpike => 10,
            MiscUsableTemplate::Statue => 10,
            MiscUsableTemplate::SilverCross => 3,
            MiscUsableTemplate::GoldCross => 5,
            MiscUsableTemplate::MithrilCross => 10,
            MiscUsableTemplate::Cross => 5,
            MiscUsableTemplate::CorkedBottle => 1,
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
            MiscUsableTemplate::FlaskOfOil => "2d6",
            MiscUsableTemplate::IronSpike => "1d1",
            MiscUsableTemplate::Statue => "1d2",
            MiscUsableTemplate::SilverCross => "1d1",
            MiscUsableTemplate::GoldCross => "1d1",
            MiscUsableTemplate::MithrilCross => "1d1",
            MiscUsableTemplate::Cross => "1d1",
            MiscUsableTemplate::CorkedBottle => "1d1",
        }
    }

    fn item_level(&self) -> u8 {
        match self {
            MiscUsableTemplate::FlaskOfOil => 10,
            MiscUsableTemplate::IronSpike => 20,
            MiscUsableTemplate::Statue => 1,
            MiscUsableTemplate::SilverCross => 25,
            MiscUsableTemplate::GoldCross => 1,
            MiscUsableTemplate::MithrilCross => 45,
            MiscUsableTemplate::Cross => 15,
            MiscUsableTemplate::CorkedBottle => 5,
        }
    }

    fn is_identified(&self) -> bool {
        false
    }
}
