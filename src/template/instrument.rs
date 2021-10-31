use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum InstrumentTemplate {
    PipesOfPeace,
    LyreOfNature,
    LuteOfTheWoods,
    HarpOfTheDruids,
}

impl InstrumentTemplate {
    pub fn iter() -> impl Iterator<Item=InstrumentTemplate> {
        [
            InstrumentTemplate::PipesOfPeace,
            InstrumentTemplate::LyreOfNature,
            InstrumentTemplate::LuteOfTheWoods,
            InstrumentTemplate::HarpOfTheDruids,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Instrument as u8,
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
            InstrumentTemplate::PipesOfPeace => "& Pipes of Peace",
            InstrumentTemplate::LyreOfNature => "& Lyre of Nature",
            InstrumentTemplate::LuteOfTheWoods => "& Lute of the Woods",
            InstrumentTemplate::HarpOfTheDruids =>"& Harp of the Druids" ,
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 0,
            InstrumentTemplate::LyreOfNature => 0,
            InstrumentTemplate::LuteOfTheWoods => 0,
            InstrumentTemplate::HarpOfTheDruids => 0x000001FF,
        }
    }

    fn flags2(&self) -> u64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 0x000003FF,
            InstrumentTemplate::LyreOfNature => 0x000FFC00,
            InstrumentTemplate::LuteOfTheWoods => 0x7FF00000,
            InstrumentTemplate::HarpOfTheDruids => 0,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 30,
            InstrumentTemplate::LyreOfNature => 105,
            InstrumentTemplate::LuteOfTheWoods => 320,
            InstrumentTemplate::HarpOfTheDruids => 850,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            InstrumentTemplate::PipesOfPeace => 258,
            InstrumentTemplate::LyreOfNature => 259,
            InstrumentTemplate::LuteOfTheWoods => 260,
            InstrumentTemplate::HarpOfTheDruids => 261,
        }
    }
}
