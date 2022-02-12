use model;
use template;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum BagTemplate {
    BagOfHolding250,
    BagOfHolding500,
    BagOfHolding1000,
    BagOfHolding1500,
    BagOfDevouring,
}

impl BagTemplate {
    pub fn vec() -> Vec<Box<dyn template::Template>> {
        vec![
            Box::new(BagTemplate::BagOfHolding250),
            Box::new(BagTemplate::BagOfHolding500),
            Box::new(BagTemplate::BagOfHolding1000),
            Box::new(BagTemplate::BagOfHolding1500),
            Box::new(BagTemplate::BagOfDevouring),
        ]
    }

    pub fn iter() -> impl Iterator<Item=Box<dyn template::Template>> {
        BagTemplate::vec().into_iter()
    }
}

impl template::Template for BagTemplate {
    fn name(&self) -> &str {
        match self {
            BagTemplate::BagOfHolding250 => "& Bag| of Holding (250)",
            BagTemplate::BagOfHolding500 => "& Bag| of Holding (500)",
            BagTemplate::BagOfHolding1000 => "& Bag| of Holding (1000)",
            BagTemplate::BagOfHolding1500 => "& Bag| of Holding (1500)",
            BagTemplate::BagOfDevouring => "& Bag| of Devouring",
        }
    }

    fn item_type(&self) -> model::ItemType { model::ItemType::Bag }

    fn flags1(&self) -> u64 {
        match self {
            BagTemplate::BagOfHolding250 => 0x04000000,
            BagTemplate::BagOfHolding500 => 0x04000000,
            BagTemplate::BagOfHolding1000 => 0x04000000,
            BagTemplate::BagOfHolding1500 => 0x04000000,
            BagTemplate::BagOfDevouring => 0x0C000000,
        }
    }

    fn flags2(&self) -> u64 { 0 }

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

    fn subtype(&self) -> i64 {
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

    fn number(&self) -> u16 { 1 }
    fn modifier_to_hit(&self) -> i16 { 0 }
    fn modifier_to_damage(&self) -> i16 { 0 }
    fn base_ac(&self) -> i16 { 0 }
    fn modifier_to_ac(&self) -> i16 { 0 }
    fn damage(&self) -> &str { "1d1" }

    fn item_level(&self) -> u8 {
        match self {
            BagTemplate::BagOfHolding250 => 20,
            BagTemplate::BagOfHolding500 => 50,
            BagTemplate::BagOfHolding1000 => 50,
            BagTemplate::BagOfHolding1500 => 35,
            BagTemplate::BagOfDevouring => 45,
        }
    }

    fn is_identified(&self) -> bool { false }
}
