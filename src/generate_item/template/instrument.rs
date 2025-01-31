use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{InstrumentSubType, ItemSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum InstrumentTemplate {
    PipesOfPeace,
    LyreOfNature,
    LuteOfTheWoods,
    HarpOfTheDruids,
}

impl InstrumentTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(InstrumentTemplate::PipesOfPeace),
            Box::new(InstrumentTemplate::LyreOfNature),
            Box::new(InstrumentTemplate::LuteOfTheWoods),
            Box::new(InstrumentTemplate::HarpOfTheDruids),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        InstrumentTemplate::vec().into_iter()
    }
}

impl ItemTemplate for InstrumentTemplate {
    fn name(&self) -> &str {
        match self {
            InstrumentTemplate::PipesOfPeace => "& Pipes of Peace",
            InstrumentTemplate::LyreOfNature => "& Lyre of Nature",
            InstrumentTemplate::LuteOfTheWoods => "& Lute of the Woods",
            InstrumentTemplate::HarpOfTheDruids => "& Harp of the Druids",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::Instrument
    }

    fn flags1(&self) -> u64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 0x000003FF,
            InstrumentTemplate::LyreOfNature => 0x000FFC00,
            InstrumentTemplate::LuteOfTheWoods => 0x7FF00000,
            InstrumentTemplate::HarpOfTheDruids => 0,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 0,
            InstrumentTemplate::LyreOfNature => 0,
            InstrumentTemplate::LuteOfTheWoods => 0,
            InstrumentTemplate::HarpOfTheDruids => 0x000001FF,
        }
    }

    fn p1(&self) -> i64 {
        0
    }

    fn cost(&self) -> i64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 30,
            InstrumentTemplate::LyreOfNature => 105,
            InstrumentTemplate::LuteOfTheWoods => 320,
            InstrumentTemplate::HarpOfTheDruids => 850,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            InstrumentTemplate::PipesOfPeace => {
                ItemSubType::Instrument(InstrumentSubType::PipesOfPeace)
            }
            InstrumentTemplate::LyreOfNature => {
                ItemSubType::Instrument(InstrumentSubType::LyreOfNature)
            }
            InstrumentTemplate::LuteOfTheWoods => {
                ItemSubType::Instrument(InstrumentSubType::LuteOfTheWoods)
            }
            InstrumentTemplate::HarpOfTheDruids => {
                ItemSubType::Instrument(InstrumentSubType::HarpOfTheDruids)
            }
        }
    }

    fn weight(&self) -> u16 {
        60
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
        "1d1"
    }
    fn item_level(&self) -> u8 {
        0
    }
    fn is_identified(&self) -> bool {
        false
    }
}
