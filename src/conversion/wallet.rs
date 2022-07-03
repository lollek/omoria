use model::Wallet;

pub fn from_int64(array: [i64; 7]) -> Wallet {
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
