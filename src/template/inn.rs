use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum LodgingAtInnTemplate {
    LodgingForOneDay,
    LodgingForThreeDays,
    LodgingForOneWeek,
    RoomAndBoardForOneDay,
}

impl LodgingAtInnTemplate {
    pub fn iter() -> impl Iterator<Item=LodgingAtInnTemplate> {
        [
            LodgingAtInnTemplate::LodgingForOneDay,
            LodgingAtInnTemplate::LodgingForThreeDays,
            LodgingAtInnTemplate::LodgingForOneWeek,
            LodgingAtInnTemplate::RoomAndBoardForOneDay,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::LodgingAtInn as u8,
            flags: 0,
            flags2: 0,
            p1: self.p1(),
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: 3000,
            number: self.number(),
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 1,
        }
    }

    fn name(&self) -> &str {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => "Lodging for one day",
            LodgingAtInnTemplate::LodgingForThreeDays => "Lodging for three days",
            LodgingAtInnTemplate::LodgingForOneWeek => "Lodging for the week",
            LodgingAtInnTemplate::RoomAndBoardForOneDay => "Room and board for one day",
        }
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

    fn subval(&self) -> i64 {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => 300,
            LodgingAtInnTemplate::LodgingForThreeDays => 302,
            LodgingAtInnTemplate::LodgingForOneWeek => 301,
            LodgingAtInnTemplate::RoomAndBoardForOneDay => 303,
        }
    }

    fn number(&self) -> u16 {
        match self {
            LodgingAtInnTemplate::LodgingForOneDay => 14,
            LodgingAtInnTemplate::LodgingForThreeDays => 1,
            LodgingAtInnTemplate::LodgingForOneWeek => 1,
            LodgingAtInnTemplate::RoomAndBoardForOneDay => 14,
        }
    }
}


