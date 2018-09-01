use libc;

use player;
use magic;

use types::{ Class, Spell };

pub fn spell(slot: usize) -> Spell {
    match player::class() {
        Class::Adventurer => magic::adventurer::spell(slot),
        Class::Bard => magic::bard::spell(slot),
        Class::Druid => magic::druid::spell(slot),
        Class::Mage => magic::mage::spell(slot),
        Class::Monk => magic::monk::spell(slot),
        Class::Paladin => magic::paladin::spell(slot),
        Class::Priest => magic::priest::spell(slot),
        Class::Rogue => magic::rogue::spell(slot),
        Class::Ranger => magic::ranger::spell(slot),
        Class::Warrior => empty_spell(),
    }
}


pub fn empty_spell() -> Spell {
    Spell {
        name: "",
        level: 99,
        mana: 99,
        fail: 0,
    }
}

#[no_mangle]
pub extern fn C_magic_spell_level(slot: libc::int32_t) -> libc::uint8_t {
    spell(slot as usize).level
}

#[no_mangle]
pub extern fn C_magic_spell_mana(slot: libc::int32_t) -> libc::uint8_t {
    spell(slot as usize).mana
}

#[no_mangle]
pub extern fn C_magic_spell_failchance(slot: libc::int32_t) -> libc::uint8_t {
    spell(slot as usize).fail
}
