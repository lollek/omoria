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

pub fn currencies_iter() -> Range<usize> {
    (Currency::Iron as usize)..(Currency::Mithril as usize) + 1
}

// Currency value in Iron
pub fn currency_value(currency: Currency) -> i64 {
    currency_value_pos(currency as usize)
}

pub fn currency_value_pos(currency_pos: usize) -> i64 {
    match currency_pos {
        1 => 1,
        2 => 4,
        3 => 20,
        4 => 240,
        5 => 960,
        6 => 12480,
        _ => panic!()
    }
}

pub struct Wallet {
    pub total: i64,
    pub iron: i64,
    pub copper: i64,
    pub silver: i64,
    pub gold: i64,
    pub platinum: i64,
    pub mithril: i64,
}

impl Wallet {
    pub fn get_pos(&self, pos: usize) -> i64 {
        match pos {
            0 => self.total,
            1 => self.iron,
            2 => self.copper,
            3 => self.silver,
            4 => self.gold,
            5 => self.platinum,
            6 => self.mithril,
            _ => panic!(),
        }
    }

    pub fn calculate_total(&mut self) {
        self.total = currencies_iter()
            .fold(0, |sum, i| sum + (self.get_pos(i) * currency_value_pos(i)))
            / currency_value(Currency::Gold)
    }
}

impl From<[i64; 7]> for Wallet {
    fn from(array: [i64; 7]) -> Wallet {
        Wallet {
            total: array[0],
            iron: array[1],
            copper: array[2],
            silver: array[3],
            gold: array[4],
            platinum: array[5],
            mithril: array[6],
        }
    }
}
