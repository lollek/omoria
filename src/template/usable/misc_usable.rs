use misc;
use model;

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

pub fn generate_misc_usable(item_level: u8, template: MiscUsableTemplate) -> model::Item {
    model::Item {
        name: misc::rs2item_name(template.name()),
        tval: template.r#type() as u8,
        flags: 0,
        flags2: template.flags2(),
        p1: template.p1(),
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

impl MiscUsableTemplate {
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

    fn r#type(&self) -> model::ItemType {
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

    fn subval(&self) -> i64 {
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
}


