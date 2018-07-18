use std::ops::Range;

pub enum Currency {
    Total = 0,
    Iron = 1,
    Copper = 2,
    Silver = 3,
    Gold = 4,
    Platinum = 5,
    Mithril = 6,
}

impl Currency {
    pub fn value(&self) -> i64 {
        match self {
            Currency::Iron => 1,
            Currency::Copper => 4,
            Currency::Silver => 20,
            Currency::Gold => 240,
            Currency::Platinum => 960,
            Currency::Mithril => 12480,
            _ => panic!()
        }
    }
}

impl From<usize> for Currency {
    fn from(value: usize) -> Currency {
        match value {
            1 => Currency::Iron,
            2 => Currency::Copper,
            3 => Currency::Silver,
            4 => Currency::Gold,
            5 => Currency::Platinum,
            6 => Currency::Mithril,
            _ => panic!(),
        }
    }
}

pub fn currencies_iter() -> Range<usize> {
    (Currency::Iron as usize)..(Currency::Mithril as usize) + 1
}

