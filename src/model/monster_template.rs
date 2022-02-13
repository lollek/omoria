use libc;

use thirdparty::serde::BigArray;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct MonsterTemplate {
    pub aaf: libc::uint8_t,           // ???
    pub ac: libc::uint8_t,            // Armor?

    pub name: [libc::c_char; 28],     // Monster name
    pub cmove: libc::uint64_t,        // ???
    pub spells: libc::uint64_t,       // Creature spells
    pub cdefense: libc::uint64_t,     // ???
    pub sleep: libc::int16_t,         // Inactive counter
    pub mexp: libc::int64_t,          // Exp value for kill
    pub speed: libc::int8_t,          // Movement speed
    pub cchar: libc::c_char,          // Character rep.
    pub hd: [libc::c_char; 7],        // Creatures hit die
    #[serde(with = "BigArray")]
    pub damage: [libc::c_char; 36],   // Type attack and damage
    pub level: libc::int8_t,          // Level of creature
    pub mr: libc::uint8_t,            // Magic Resistance
}
