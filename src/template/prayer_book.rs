use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum PrayerBookTemplate {
    BeginnersHandbook,
    WordsOfWisdom,
    ChantsAndBlessings,
    ExorcismAndDispelling,
}

impl PrayerBookTemplate {
    pub fn iter() -> impl Iterator<Item=PrayerBookTemplate> {
        [
            PrayerBookTemplate::BeginnersHandbook,
            PrayerBookTemplate::WordsOfWisdom,
            PrayerBookTemplate::ChantsAndBlessings,
            PrayerBookTemplate::ExorcismAndDispelling,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::PrayerBook as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: 0,
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: 60,
            number: 1,
            tohit: -100,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("1d1"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            PrayerBookTemplate::BeginnersHandbook => "& Holy Book of Prayers [Beginners Handbook]",
            PrayerBookTemplate::WordsOfWisdom => "& Holy Book of Prayers [Words of Wisdom]",
            PrayerBookTemplate::ChantsAndBlessings => "& Holy Book of Prayers [Chants and Blessings]",
            PrayerBookTemplate::ExorcismAndDispelling => "& Holy Book of Prayers [Exorcism and Dispelling]",
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 0,
            PrayerBookTemplate::WordsOfWisdom => 0,
            PrayerBookTemplate::ChantsAndBlessings => 0x00000001,
            PrayerBookTemplate::ExorcismAndDispelling => 0x000001FE,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 0x000000FF,
            PrayerBookTemplate::WordsOfWisdom => 0x0007FF00,
            PrayerBookTemplate::ChantsAndBlessings => 0x7FF80000,
            PrayerBookTemplate::ExorcismAndDispelling => 0,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 25,
            PrayerBookTemplate::WordsOfWisdom => 100,
            PrayerBookTemplate::ChantsAndBlessings => 300,
            PrayerBookTemplate::ExorcismAndDispelling => 900,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            PrayerBookTemplate::BeginnersHandbook => 258,
            PrayerBookTemplate::WordsOfWisdom => 259,
            PrayerBookTemplate::ChantsAndBlessings => 260,
            PrayerBookTemplate::ExorcismAndDispelling => 261,
        }
    }
}

