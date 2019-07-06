use regex::Regex;
use thirdparty::serde::{ BigArray };

use misc;
use player;
use random;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Creature { // creature_type
    pub aaf: libc::uint8_t, // Area effect radius
    pub ac: libc::uint8_t, // AC
    pub name: [libc::c_char; 28], // Description of creature
    pub cmove: libc::uint64_t, // Bit field
    pub spells: libc::uint64_t, // Creature spells
    pub cdefense: libc::uint64_t, // Bit field
    pub sleep: libc::int16_t, // Inactive counter
    pub mexp: libc::int64_t, // Exp value for kill
    pub speed: libc::int8_t, // Movement speed
    pub cchar: libc::c_char, // Character rep
    pub hd: [libc::c_char; 7], // Creature hit die
    #[serde(with = "BigArray")]
    pub damage: [libc::c_char; 36], // Type attack and damage
    pub level: libc::int8_t, // Level of creature
    pub mr: libc::uint8_t, // Magic resistance
}

impl Creature {
    pub fn calculate_hp(&self) -> i16 {
        if self.cdefense & 0x4000 != 0 {
            return self.max_hp();
        }

        let (num, die) = self.hp_as_num_die();
        random::rand_rep(num, die) as i16
    }

    fn max_hp(&self) -> i16 {
        let (num, die) = self.hp_as_num_die();
        (num * die) as i16
    }

    fn hp_as_num_die(&self) -> (i64, i64) {
        let re = Regex::new(r"(\d+)d(\d+)").unwrap();
        let hp = self.hp_as_string();
        let captures = re.captures(&hp).unwrap();
        (captures.get(1).unwrap().as_str().parse().ok().unwrap(),
        captures.get(2).unwrap().as_str().parse().ok().unwrap())
    }

    fn hp_as_string(&self) -> String {
        misc::c_i8_array_to_rust_string(self.hd.to_vec())
    }

    pub fn calculate_rounds_asleep(&self) -> i16 {
        (self.sleep as f32 / 5.0) as i16 + random::randint(self.sleep as i64) as i16
    }

    pub fn calculate_speed(&self) -> i8 {
        self.speed + player::speed() as i8
    }
}
