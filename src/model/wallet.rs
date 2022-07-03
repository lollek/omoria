use data;
use model::Currency;

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
    pub fn get_pos(&self, pos: Currency) -> i64 {
        match pos {
            Currency::Iron => self.iron,
            Currency::Copper => self.copper,
            Currency::Silver => self.silver,
            Currency::Gold => self.gold,
            Currency::Platinum => self.platinum,
            Currency::Mithril => self.mithril,
        }
    }

    pub fn calculate_total(&mut self) {
        self.total = Currency::iter().fold(0, |sum, i| {
            sum + (self.get_pos(i) * data::currency::value(&i))
        }) / data::currency::value(&Currency::Gold)
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
