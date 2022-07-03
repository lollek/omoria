use data;
use magic;
use player;

use model::{Class, Spell};

pub fn spells_iter() -> impl Iterator<Item = usize> {
    0..40
}

pub fn spell(slot: usize) -> Spell {
    match player::class() {
        Class::Adventurer => magic::adventurer::spell(slot),
        Class::Bard => magic::bard::spell(slot),
        Class::Druid => magic::druid::spell(slot),
        Class::Wizard => magic::mage::spell(slot),
        Class::Monk => magic::monk::spell(slot),
        Class::Paladin => magic::paladin::spell(slot),
        Class::Cleric => magic::priest::spell(slot),
        Class::Rogue => magic::rogue::spell(slot),
        Class::Ranger => magic::ranger::spell(slot),
        Class::Fighter => empty_spell(),
        Class::Barbarian => empty_spell(),
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

pub fn gain_mana_from_level_up() {
    if let Some(magic) = data::class::magic_type(&player::class()) {
        player::modify_max_mp(player::modifier_from_stat(data::magic::modifier_stat(&magic)));
    }
}
