use crate::data;
use crate::model::{Currency, Wallet};

pub fn calculate_total(wallet: &Wallet) -> i64 {
    Currency::iter().fold(0, |sum, i| {
        sum + (wallet.get_pos(i) * data::currency::value(&i))
    }) / data::currency::value(&Currency::Gold)
}
