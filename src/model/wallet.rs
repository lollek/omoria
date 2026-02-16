use crate::model::Currency;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
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
    pub fn new() -> Self {
        Wallet {
            total: 0,
            iron: 0,
            copper: 0,
            silver: 0,
            gold: 0,
            platinum: 0,
            mithril: 0,
        }
    }

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
}

impl From<[i64; 7]> for Wallet {
    fn from(array: [i64; 7]) -> Self {
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

impl From<Wallet> for [i64; 7] {
    fn from(wallet: Wallet) -> Self {
        [
            wallet.total,
            wallet.iron,
            wallet.copper,
            wallet.silver,
            wallet.gold,
            wallet.platinum,
            wallet.mithril,
        ]
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Wallet;

    #[test]
    fn wallet_converts_from_i64_array_in_expected_positions() {
        let array = [10, 1, 2, 3, 4, 5, 6];

        let wallet: Wallet = array.into();

        assert_eq!(wallet.total, 10);
        assert_eq!(wallet.iron, 1);
        assert_eq!(wallet.copper, 2);
        assert_eq!(wallet.silver, 3);
        assert_eq!(wallet.gold, 4);
        assert_eq!(wallet.platinum, 5);
        assert_eq!(wallet.mithril, 6);
    }

    #[test]
    fn wallet_converts_into_i64_array_in_expected_positions() {
        let wallet = Wallet {
            total: 10,
            iron: 1,
            copper: 2,
            silver: 3,
            gold: 4,
            platinum: 5,
            mithril: 6,
        };

        let array: [i64; 7] = wallet.into();

        assert_eq!(array, [10, 1, 2, 3, 4, 5, 6]);
    }
}
