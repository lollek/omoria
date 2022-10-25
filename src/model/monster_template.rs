use libc;

use crate::thirdparty::serde::BigArray;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct MonsterTemplate {
    pub aaf: u8,           // ???
    pub ac: u8,            // Armor?

    pub name: [libc::c_char; 28],     // Monster name
    pub cmove: u64,        // ???
    pub spells: u64,       // Creature spells
    pub cdefense: u64,     // ???
    pub sleep: i16,         // Inactive counter
    pub mexp: i64,          // Exp value for kill
    pub speed: i8,          // Movement speed
    pub cchar: libc::c_char,          // Character rep.
    pub hd: [libc::c_char; 7],        // Creatures hit die
    #[serde(with = "BigArray")]
    pub damage: [libc::c_char; 36],   // Type attack and damage
    pub level: i8,          // Level of creature
    pub mr: u8,            // Magic Resistance
}
