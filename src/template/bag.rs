use misc;
use model;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BagTemplate {
    BagOfHolding250,
    BagOfHolding500,
    BagOfHolding1000,
    BagOfHolding1500,
    BagOfDevouring,
}

impl BagTemplate {
    pub fn iter() -> impl Iterator<Item=BagTemplate> {
        [
            BagTemplate::BagOfHolding250,
            BagTemplate::BagOfHolding500,
            BagTemplate::BagOfHolding1000,
            BagTemplate::BagOfHolding1500,
            BagTemplate::BagOfDevouring,
        ].iter().copied()
    }

    pub fn create(&self) -> model::Item {
        model::Item {
            name: misc::rs2item_name(self.name()),
            tval: model::ItemType::Bag as u8,
            flags: self.flags1(),
            flags2: 0,
            p1: self.p1(),
            cost: self.cost() * model::Currency::Gold.value(),
            subval: self.subval(),
            weight: self.weight(),
            number: 1,
            tohit: 0,
            todam: 0,
            ac: 0,
            toac: 0,
            damage: misc::rs2item_damage("0d0"),
            level: 0,
            identified: 0,
        }
    }

    fn name(&self) -> &str {
        match self {
            BagTemplate::BagOfHolding250 => "& %N Bag| of Holding (250)",
            BagTemplate::BagOfHolding500 => "& %N Bag| of Holding (500)",
            BagTemplate::BagOfHolding1000 => "& %N Bag| of Holding (1000)",
            BagTemplate::BagOfHolding1500 => "& %N Bag| of Holding (1500)",
            BagTemplate::BagOfDevouring => "& %N Bag| of Devouring",
        }
    }

    fn flags1(&self) -> u64 {
        match self {
            BagTemplate::BagOfHolding250 => 0x04000000,
            BagTemplate::BagOfHolding500 => 0x04000000,
            BagTemplate::BagOfHolding1000 => 0x04000000,
            BagTemplate::BagOfHolding1500 => 0x04000000,
            BagTemplate::BagOfDevouring => 0x0C000000,
        }
    }

    fn p1(&self) -> i64 {
        match self {
            BagTemplate::BagOfHolding250 => 25000,
            BagTemplate::BagOfHolding500 => 50000,
            BagTemplate::BagOfHolding1000 => 100000,
            BagTemplate::BagOfHolding1500 => 150000,
            BagTemplate::BagOfDevouring => 150000,
        }
    }

    fn cost(&self) -> i64 {
        match self {
            BagTemplate::BagOfHolding250 => 1000,
            BagTemplate::BagOfHolding500 => 2000,
            BagTemplate::BagOfHolding1000 => 5000,
            BagTemplate::BagOfHolding1500 => 7000,
            BagTemplate::BagOfDevouring => 0,
        }
    }

    fn subval(&self) -> i64 {
        match self {
            BagTemplate::BagOfHolding250 => 1,
            BagTemplate::BagOfHolding500 => 2,
            BagTemplate::BagOfHolding1000 => 3,
            BagTemplate::BagOfHolding1500 => 3,
            BagTemplate::BagOfDevouring => 4,
        }
    }

    fn weight(&self) -> u16 {
        match self {
            BagTemplate::BagOfHolding250 => 50,
            BagTemplate::BagOfHolding500 => 100,
            BagTemplate::BagOfHolding1000 => 200,
            BagTemplate::BagOfHolding1500 => 300,
            BagTemplate::BagOfDevouring => 100,
        }
    }

    fn level(&self) -> u8 {
        match self {
            BagTemplate::BagOfHolding250 => 20,
            BagTemplate::BagOfHolding500 => 50,
            BagTemplate::BagOfHolding1000 => 50,
            BagTemplate::BagOfHolding1500 => 35,
            BagTemplate::BagOfDevouring => 45,
        }
    }
}