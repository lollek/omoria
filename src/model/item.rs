use crate::thirdparty::serde::BigArray;

use crate::conversion;
use crate::model::ItemType;
use crate::model::{Damage, Name};

pub enum ChestFlags1 {
    Locked = 0x00000001,
    // 1d4 damage, may lose strength
    PoisonNeedle1 = 0x00000010,
    // 1d6 damage, poisons player
    PoisonNeedle2 = 0x00000020,
    GasTrap = 0x00000040,
    // 5d8 damage, destroys chest
    ExplosiveTrap = 0x00000080,
    SummoningRunes = 0x00000100,
}

pub enum MiscUsableFlag1 {
    Turning = 0x00000001,
    DemonDispelling = 0x00000002,
    SummonUndead = 0x00000004,
    SummonDemon = 0x00000008,
    ContainingDjinni = 0x00000010,
    ContainingDemons = 0x00000020,
    MajorSummonUndead = 0x00000100,
    MajorSummonDemon = 0x00000200,
    LifeGiving = 0x00000400,
}

pub enum WornFlag1 {
    GivesStrength = 0x00000001,
    GivesDexterity = 0x00000002,
    GivesConstitution = 0x00000004,
    GivesIntelligence = 0x00000008,
    GivesWisdom = 0x00000010,
    GivesCharisma = 0x00000020,
    // + to search, - to perception (amount in p1)
    Searching = 0x00000040,
    SlowDigestion = 0x00000080,
    Stealth = 0x00000100,
    AggravateMonsters = 0x00000200,
    RandomTeleportation = 0x00000400,
    Regeneration = 0x00000800,
    // Amount in p1
    IncreasedSpeed = 0x00001000,
    // Extra damage to dragons
    SlayDragon = 0x00002000,
    // Extra damage to monsters
    SlayMonster = 0x00004000,
    // Extra damage to evil creatures
    SlayEvil = 0x00008000,
    // Extra damage to undead
    SlayUndead = 0x00010000,
    // Extra damage to cold based creatures
    SlayCold = 0x00020000,
    // Extra damage to fire based creatures
    SlayFire = 0x00040000,
    ResistFire = 0x00080000,
    ResistAcid = 0x00100000,
    ResistCold = 0x00200000,
    // See p1 for specific stat
    ResistStatDrain = 0x00400000,
    ResistParalysis = 0x00800000,
    SeeInvisible = 0x01000000,
    ResistLightning = 0x02000000,
    FeatherFall = 0x04000000,
    //Blindness = 0x08000000,
    //Timidness = 0x10000000,
    // See p1 for amount
    ImprovedTunneling = 0x20000000,
    //Infravision = 0x40000000,
    Cursed = 0x80000000,
}

pub enum WornFlag2 {
    SlayDemon = 0x00000001,
    // Lessed Slay Undead?
    SoulSword = 0x00000002,
    SlayRegenerators = 0x00000004,
    // See p1 for amount
    //ImprovedSwimming = 0x00000008,
    MagicProof = 0x00000010,
    //ImprovedDisarming = 0x00000020,
    //ImprovedCriticals = 0x00000040,
    BadReputation = 0x00200000,
    //Hunger = 0x00400000,
    // Removes flags for other worn gems (???)
    //NegativeGemFlags = 0x00800000,
    ImprovedCarrying = 0x01000000,
    // Related to quivers and such (???)
    Holding = 0x04000000,
    // Bag of devouring
    Swallowing = 0x08000000,
    // Destroys bags
    Sharp = 0x10000000,
    Blackmarket = 0x20000000,
    //Insured = 0x40000000,
    //KnownCursed = 0x80000000,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
// For more info. Se item_guide.txt
// TODO: tval + subval needs bit for is_identified
pub struct Item {
    // treasure_type
    // Object name. See below for rules on names.
    #[serde(with = "BigArray")]
    pub name: Name,

    // Object type. Literally, is what kind of object it is.
    pub tval: u8,

    // Flags define an item's properties. the meaning of flag values can
    // depend on the item's tval.  note that the original game designers
    // decided to use bitwise flags (a good design), but came up with more
    // than 32 different properties for some item types (e.g.  potions)
    // and so they had to give items a second flag member to hold them all.
    pub flags2: u64,
    pub flags: u64,

    // A catch-all member for associating some value of interest
    // with a particular item.  e.g. for wands, staves, gems,
    // it's the number of charges, while for certain magical
    // weapons, it can be a damage multiplier
    pub p1: i64,

    // The item's nominal cost in iron pieces.
    pub cost: i64,

    // A sub-category value.  the semantics of subval also depend
    // in part on the value of tval.
    pub subval: i64,

    // Weight in some ill-defined unit, possibly gold pieces, but
    // possibly in fifths of a gold piece.
    pub weight: u16,

    // for countable items (arrows, scrolls, etc), how many of
    // them you have.
    pub number: u16,

    // the next four all apply to items that can be worn or wielded (i.e. weapons
    // and armor)
    pub tohit: i16, // An item's magical + to hit
    pub todam: i16, // An item's magical + to damage
    pub ac: i16,    // An item's normal armor class value.
    pub toac: i16,  // An item's magical + to AC.

    // the amount of damage an item does to monster.  everything
    // should have a damage value, even if it's just "0d0".
    pub damage: Damage,

    // a vague measurement of how strong an item's magic is.
    pub level: i8,

    // is this item identified?
    pub identified: u8,
}

impl Item {
    pub fn item_type(&self) -> ItemType {
        conversion::item_type::from_usize(self.tval.into())
            .unwrap_or_else(|| panic!("expected item type from tval {}", self.tval))
    }

    pub fn apply_chestflag1(&mut self, flag: ChestFlags1) {
        self.flags |= flag as u64;
    }

    pub fn apply_misc_usable_flag(&mut self, flag: MiscUsableFlag1) {
        self.flags |= flag as u64;
    }

    pub fn has_misc_usable_flag(&self, flag: MiscUsableFlag1) -> bool {
        self.flags & flag as u64 != 0
    }

    pub fn apply_wornflag1(&mut self, flag: WornFlag1) {
        self.flags |= flag as u64;
    }

    pub fn has_wornflag1(&self, flag: WornFlag1) -> bool {
        self.flags & flag as u64 != 0
    }

    pub fn apply_wornflag2(&mut self, flag: WornFlag2) {
        self.flags2 |= flag as u64;
    }

    pub fn has_wornflag2(&self, flag: WornFlag2) -> bool {
        self.flags2 & flag as u64 != 0
    }

    pub fn is_cursed(&self) -> bool {
        self.flags & (WornFlag1::Cursed as u64) != 0
    }

    pub fn set_cursed(&mut self, yn: bool) {
       if !yn {
           panic!("Not implemented yet");
       }
        self.apply_wornflag1(WornFlag1::Cursed);
    }

    pub fn is_identified(&self) -> bool {
        self.identified != 0
    }

    pub fn set_identified(&mut self, yn: bool) {
        self.identified = if yn { 255 } else { 0 };
    }
}
