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

}