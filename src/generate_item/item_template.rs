use crate::conversion::item_subtype;
use crate::conversion::item_type;
use crate::data;
use crate::misc;
use crate::model::item_subtype::ItemSubType;
use crate::model::Currency;
use crate::model::Item;
use crate::model::ItemType;

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
    //Stealth = 0x00000100,
    AggravateMonsters = 0x00000200,
    RandomTeleportation = 0x00000400,
    //Regeneration = 0x00000800,
    // Amount in p1
    IncreasedSpeed = 0x00001000,
    // Extra damage to dragons
    //SlayDragon = 0x00002000,
    // Extra damage to monsters
    //SlayMonster = 0x00004000,
    // Extra damage to evil creatures
    //SlayEvil = 0x00008000,
    // Extra damage to undead
    //SlayUndead = 0x00010000,
    // Extra damage to cold based creatures
    //SlayCold = 0x00020000,
    // Extra damage to fire based creatures
    //SlayFire = 0x00040000,
    ResistFire = 0x00080000,
    ResistAcid = 0x00100000,
    ResistCold = 0x00200000,
    // See p1 for specific stat
    ResistStatDrain = 0x00400000,
    ResistParalysis = 0x00800000,
    SeeInvisible = 0x01000000,
    //ResistLightning = 0x02000000,
    FeatherFall = 0x04000000,
    //Blindness = 0x08000000,
    //Timidness = 0x10000000,
    // See p1 for amount
    ImprovedTunneling = 0x20000000,
    //Infravision = 0x40000000,
    Cursed = 0x80000000,
}

pub enum WornFlag2 {
    //SlayDemon = 0x00000001,
    // Lessed Slay Undead?
    //SoulSword = 0x00000002,
    //SlayRegenerators = 0x00000004,
    // See p1 for amount
    //ImprovedSwimming = 0x00000008,
    //ImprovedSavingThrows = 0x00000010,
    //ImprovedDisarming = 0x00000020,
    //ImprovedCriticals = 0x00000040,
    //BadReputation = 0x00200000,
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
    //Blackmarket = 0x20000000,
    //Insured = 0x40000000,
    //KnownCursed = 0x80000000,
}

pub trait ItemTemplate {
    fn create(&self) -> Item {
        Item {
            name: misc::rs2item_name(self.name()),
            tval: item_type::to_usize(self.item_type()) as u8,
            flags: self.flags1(),
            flags2: self.flags2(),
            p1: self.p1(),
            cost: self.cost() * data::currency::value(&Currency::Gold),
            subval: item_subtype::to_usize(self.subtype()) as i64,
            weight: self.weight(),
            number: self.number(),
            tohit: self.modifier_to_hit(),
            todam: self.modifier_to_damage(),
            ac: self.base_ac(),
            toac: self.modifier_to_ac(),
            damage: misc::rs2item_damage(self.damage()),
            level: self.item_level() as i8,
            identified: if self.is_identified() { 1 } else { 0 },
        }
    }

    fn name(&self) -> &str;
    fn item_type(&self) -> ItemType;
    fn flags1(&self) -> u64;
    fn flags2(&self) -> u64;
    fn p1(&self) -> i64;
    fn cost(&self) -> i64;
    fn subtype(&self) -> ItemSubType;
    fn weight(&self) -> u16;
    fn number(&self) -> u16;
    fn modifier_to_hit(&self) -> i16;
    fn modifier_to_damage(&self) -> i16;
    fn base_ac(&self) -> i16;
    fn modifier_to_ac(&self) -> i16;
    fn damage(&self) -> &str;
    fn item_level(&self) -> u8;
    fn is_identified(&self) -> bool;
}
