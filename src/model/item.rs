use model::ItemType;
use libc;
use model::{ Damage, Name };

use thirdparty::serde::BigArray;

#[repr(C)]
#[derive(Copy, Clone, Serialize, Deserialize)]
// For more info. Se item_guide.txt
// TODO: tval + subval needs bit for is_identified
pub struct Item { // treasure_type
    // Object name. See below for rules on names.
    #[serde(with = "BigArray")]
    pub name: Name,

    // Object type. Literally, is what kind of object it is.
    pub tval: libc::uint8_t,

    // Flags define an item's properties. the meaning of flag values can
    // depend on the item's tval.  note that the original game designers
    // decided to use bitwise flags (a good design), but came up with more
    // than 32 different properties for some item types (e.g.  potions)
    // and so they had to give items a second flag member to hold them all.
    pub flags2: libc::uint64_t,
    pub flags: libc::uint64_t,

    // A catch-all member for associating some value of interest
    // with a particular item.  e.g. for wands, staves, gems,
    // it's the number of charges, while for certain magical
    // weapons, it can be a damage multiplier
    pub p1: libc::int64_t,

    // The item's nominal cost in iron pieces.
    pub cost: libc::int64_t,

    // A sub-category value.  the semantics of subval also depend
    // in part on the value of tval.
    pub subval: libc::int64_t,

    // Weight in some ill-defined unit, possibly gold pieces, but
    // possibly in fifths of a gold piece.
    pub weight: libc::uint16_t,

    // for countable items (arrows, scrolls, etc), how many of
    // them you have.
    pub number: libc::uint16_t,

    // the next four all apply to items that can be worn or wielded (i.e. weapons
    // and armor)
    pub tohit: libc::int16_t,       // An item's magical + to hit
    pub todam: libc::int16_t,       // An item's magical + to damage
    pub ac: libc::int16_t,          // An item's normal armor class value.
    pub toac: libc::int16_t,        // An item's magical + to AC.

    // the amount of damage an item does to monster.  everything
    // should have a damage value, even if it's just "0d0".
    pub damage: Damage,

    // a vague measurement of how strong an item's magic is.
    pub level: libc::int8_t,

    // is this item identified?
    pub identified: libc::uint8_t,
}

impl Item {

    pub fn item_type(&self) -> ItemType {
        ItemType::from(self.tval)
    }

    pub fn is_identified(&self) -> bool {
        self.identified != 0
    }
}
