use libc;

use crate::model::{Class, GameTime, Player, PlayerFlags, Race, Sex, Time, Wallet};

#[repr(C)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRecord {
    pub uid: i64,
    pub account: i64,
    pub money: Wallet,
    pub birth: GameTime,
    pub cur_age: GameTime,
    pub cur_quest: u16,
    pub quests: u8,
    pub claim_check: i64,
    pub play_tm: Time,
    pub name: String,
    pub race: Race,
    pub sex: Sex,
    pub class: Class,
    pub history: Vec<String>,
    pub cheated: u8,
    pub age: u16,
    pub ht: u16,
    pub wt: u16,
    pub sc: i16,
    pub max_exp: i64,
    pub exp: i64,
    pub rep: i64,
    pub lev: u16,
    pub max_lev: u16,
    pub expfact: libc::c_float,
    pub fos: i16,
    pub stl: i16,
    pub bth: i16,
    pub bthb: i16,
    pub mana: i16,
    pub cmana: libc::c_float,
    pub ptohit: i16,
    pub ptodam: i16,
    pub pac: i16,
    pub ptoac: i16,
    pub dis_th: i16,
    pub dis_td: i16,
    pub dis_ac: i16,
    pub dis_tac: i16,
    pub disarm: i16,
    pub save: i16,
    pub inven_weight: libc::c_long,
    pub flags: PlayerFlags,
    pub player: Player,
    pub char_row: libc::c_long,
    pub char_col: libc::c_long,
}
impl PlayerRecord {
    pub fn new() -> Self {
        PlayerRecord {
            uid: 0,
            account: 0,
            money: Wallet::default(),
            birth: Default::default(),
            cur_age: Default::default(),
            cur_quest: 0,
            quests: 0,
            claim_check: 0,
            play_tm: Time::default(),
            name: "".to_string(),
            race: Race::Human,
            sex: Sex::Female,
            class: Class::Fighter,
            history: vec![],
            cheated: 0,
            age: 0,
            ht: 0,
            wt: 0,
            sc: 0,
            max_exp: 0,
            exp: 0,
            rep: 0,
            lev: 0,
            max_lev: 0,
            expfact: 0.0,
            fos: 0,
            stl: 0,
            bth: 0,
            bthb: 0,
            mana: 0,
            cmana: 0.0,
            ptohit: 0,
            ptodam: 0,
            pac: 0,
            ptoac: 0,
            dis_th: 0,
            dis_td: 0,
            dis_ac: 0,
            dis_tac: 0,
            disarm: 0,
            save: 0,
            inven_weight: 0,
            flags: PlayerFlags::default(),
            player: Player::default(),
            char_row: 0,
            char_col: 0,
        }
    }
}

impl Default for PlayerRecord {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializeable() {
        serde_json::to_string(&PlayerRecord::default()).expect("Failed to serialize player");
    }
}
