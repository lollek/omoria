use types::{Currency, currencies_iter};

#[derive(Copy, Clone, Serialize, Deserialize)]
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
            .fold(0, |sum, i| sum + (self.get_pos(i) * Currency::from(i).value()))
            / Currency::Gold.value()
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
