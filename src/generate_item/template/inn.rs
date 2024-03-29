use super::super::item_template::ItemTemplate;
use crate::model::{
    self,
    item_subtype::{ItemSubType, LodgingAtInnSubType},
};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LodgingAtInnTemplate {
    LodgingForOneDay,
    LodgingForThreeDays,
    LodgingForOneWeek,
    RoomAndBoardForOneDay,
}

impl LodgingAtInnTemplate {
    pub fn vec() -> Vec<Box<dyn ItemTemplate>> {
        vec![
            Box::new(LodgingAtInnTemplate::LodgingForOneDay),
            Box::new(LodgingAtInnTemplate::LodgingForThreeDays),
            Box::new(LodgingAtInnTemplate::LodgingForOneWeek),
            Box::new(LodgingAtInnTemplate::RoomAndBoardForOneDay),
        ]
    }

    pub fn iter() -> impl Iterator<Item = Box<dyn ItemTemplate>> {
        LodgingAtInnTemplate::vec().into_iter()
    }
}

impl ItemTemplate for LodgingAtInnTemplate {
    fn name(&self) -> &str {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => "Lodging for one day",
            LodgingAtInnTemplate::LodgingForThreeDays => "Lodging for three days",
            LodgingAtInnTemplate::LodgingForOneWeek => "Lodging for the week",
            LodgingAtInnTemplate::RoomAndBoardForOneDay => "Room and board for one day",
        }
    }

    fn item_type(&self) -> model::ItemType {
        model::ItemType::LodgingAtInn
    }
    fn flags1(&self) -> u64 {
        0
    }
    fn flags2(&self) -> u64 {
        0
    }

    fn p1(&self) -> i64 {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => 1,
            LodgingAtInnTemplate::LodgingForThreeDays => 3,
            LodgingAtInnTemplate::LodgingForOneWeek => 7,
            LodgingAtInnTemplate::RoomAndBoardForOneDay => 1,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => 50,
            LodgingAtInnTemplate::LodgingForThreeDays => 120,
            LodgingAtInnTemplate::LodgingForOneWeek => 200,
            LodgingAtInnTemplate::RoomAndBoardForOneDay => 70,
        }
    }

    fn subtype(&self) -> ItemSubType {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => {
                ItemSubType::LodgingAtInn(LodgingAtInnSubType::LodgingForOneDay)
            }
            LodgingAtInnTemplate::LodgingForThreeDays => {
                ItemSubType::LodgingAtInn(LodgingAtInnSubType::LodgingForThreeDays)
            }
            LodgingAtInnTemplate::LodgingForOneWeek => {
                ItemSubType::LodgingAtInn(LodgingAtInnSubType::LodgingForOneWeek)
            }
            LodgingAtInnTemplate::RoomAndBoardForOneDay => {
                ItemSubType::LodgingAtInn(LodgingAtInnSubType::RoomAndBoardForOneDay)
            }
        }
    }

    fn weight(&self) -> u16 {
        3000
    }

    fn number(&self) -> u16 {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => 14,
            LodgingAtInnTemplate::LodgingForThreeDays => 1,
            LodgingAtInnTemplate::LodgingForOneWeek => 1,
            LodgingAtInnTemplate::RoomAndBoardForOneDay => 14,
        }
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
        1
    }
    fn is_identified(&self) -> bool {
        true
    }
}
