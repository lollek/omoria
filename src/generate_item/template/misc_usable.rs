use crate::generate_item::item_template::{default_create, ItemQuality};
use crate::misc::rs2item_name;
use super::super::item_template::ItemTemplate;
use crate::model::{self, item_subtype::{FlaskOfOilSubType, ItemSubType, MiscUsableSubType, SpikeSubType}, Item};
use crate::model::MiscUsableFlag1::{ContainingDemons, ContainingDjinni, DemonDispelling, LifeGiving, MajorSummonDemon, MajorSummonUndead, SummonDemon, SummonUndead, Turning};
use crate::rng::randint;

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

    pub fn apply_cross_of_turning(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of turning", self.name()));
        item.apply_misc_usable_flag(Turning);
        item.p1 = randint(item.p1 * 2) + 2;
        item.cost = item.p1 * 20_000;
    }

    pub fn apply_cross_of_demon_dispelling(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of demon dispelling", self.name()));
        item.apply_misc_usable_flag(DemonDispelling);
        item.p1 = randint(9);
        item.cost = item.p1 * 50_000;
    }

    pub fn apply_cross_of_summon_undead(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of summon undead", self.name()));
        item.apply_misc_usable_flag(SummonUndead);
        item.cost = 0;
        item.p1 = 2;
    }

    pub fn apply_cross_of_summon_demon(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} of summon demon", self.name()));
        item.apply_misc_usable_flag(SummonDemon);
        item.cost = 0;
        item.p1 = 2;
    }

    pub fn apply_bottle_of_djinni(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} containing a djinni", self.name()));
        item.apply_misc_usable_flag(ContainingDjinni);
        item.cost = 200_000;
        item.p1 = 1;
    }

    pub fn apply_bottle_of_demons(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} containing some demons", self.name()));
        item.apply_misc_usable_flag(ContainingDemons);
        item.cost = 0;
        item.p1 = 1;
    }

    pub fn apply_statue_of_summon_undead(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} major of undead summoning", self.name()));
        item.apply_misc_usable_flag(MajorSummonUndead);
        item.cost = 0;
        item.p1 = randint(4) + 2;
    }

    pub fn apply_statue_of_summon_demon(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} major of demon summoning", self.name()));
        item.apply_misc_usable_flag(MajorSummonDemon);
        item.cost = 0;
        item.p1 = randint(3) + 1;
    }

    pub fn apply_statue_of_give_life(&self, item: &mut Item) {
        item.name = rs2item_name(&format!("{} life giving", self.name()));
        item.apply_misc_usable_flag(LifeGiving);
        item.cost = 900000;
        item.p1 = randint(5) + 3;
    }
}

impl ItemTemplate for MiscUsableTemplate {
    fn create(&self, item_quality: ItemQuality, _item_level: u8) -> Item {
        let mut item = default_create(self, item_quality);
        if item_quality == ItemQuality::Special {
            match self {
                MiscUsableTemplate::Statue => {
                    match randint(3) {
                        1 => self.apply_statue_of_summon_undead(&mut item),
                        2 => self.apply_statue_of_summon_demon(&mut item),
                        _ => self.apply_statue_of_give_life(&mut item),
                    }
                },
                MiscUsableTemplate::SilverCross |
                MiscUsableTemplate::GoldCross |
                MiscUsableTemplate::MithrilCross |
                MiscUsableTemplate::Cross => {
                    match randint(4) {
                        1 => self.apply_cross_of_turning(&mut item),
                        2 => self.apply_cross_of_demon_dispelling(&mut item),
                        3 => self.apply_cross_of_summon_undead(&mut item),
                        _ => self.apply_cross_of_summon_demon(&mut item),
                    }
                },
                MiscUsableTemplate::CorkedBottle => {
                    match randint(3) {
                        1 | 2 => self.apply_bottle_of_demons(&mut item),
                        _ => self.apply_bottle_of_djinni(&mut item),
                    }
                }
                _ => {}
            }
        }
        item
    }

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

    fn flags2(&self) -> u64 {
        0
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

    fn subtype(&self) -> ItemSubType {
        match self {
            MiscUsableTemplate::FlaskOfOil => {
                ItemSubType::FlaskOfOil(FlaskOfOilSubType::FlaskOfOil)
            }
            MiscUsableTemplate::IronSpike => ItemSubType::Spike(SpikeSubType::IronSpike),
            MiscUsableTemplate::Statue => ItemSubType::MiscUsable(MiscUsableSubType::Statue),
            MiscUsableTemplate::SilverCross => {
                ItemSubType::MiscUsable(MiscUsableSubType::SilverCross)
            }
            MiscUsableTemplate::GoldCross => ItemSubType::MiscUsable(MiscUsableSubType::GoldCross),
            MiscUsableTemplate::MithrilCross => {
                ItemSubType::MiscUsable(MiscUsableSubType::MithrilCross)
            }
            MiscUsableTemplate::Cross => ItemSubType::MiscUsable(MiscUsableSubType::Cross),
            MiscUsableTemplate::CorkedBottle => {
                ItemSubType::MiscUsable(MiscUsableSubType::CorkedBottle)
            }
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
