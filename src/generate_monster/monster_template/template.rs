//! Monster template struct.
//!
//! Static definition of a monster type, mirroring `monster_template_t` from C.

use super::MonsterAttribute;

/// Monster template (static definition).
///
/// This struct mirrors the C `monster_template_t` layout.
/// Field order and types match `monster_template.h`.
#[derive(Debug, Clone)]
pub struct MonsterTemplate {
    /// Area effect radius for spells/breath.
    pub area_effect_radius: u8,
    /// Armor class.
    pub ac: u8,
    /// Monster name (e.g., "Kobold", "Balrog").
    pub name: &'static str,
    /// Movement/behavior bit field.
    pub cmove: u64,
    /// Spell bit field.
    pub spells: u64,
    /// Defense/vulnerability bit field.
    pub cdefense: u64,
    /// Inactive counter when spawned.
    pub sleep: i16,
    /// Experience value for kill.
    pub mexp: i64,
    /// Movement speed.
    pub speed: i8,
    /// Symbol displayed on map.
    pub symbol: char,
    /// Hit dice (e.g., "20d8").
    pub hit_die: &'static str,
    /// Attack damage string (e.g., "1 1 3d3|1 1 4d4").
    pub damage: &'static str,
    /// Minimum dungeon level where monster appears.
    pub level: i8,
    /// Magic resistance (0-255).
    pub magic_resistance: u8,
    /// Whether monster can multiply/reproduce.
    pub multiplies: bool,
    /// Whether monster can move (false = stationary).
    pub can_move: bool,
}

impl MonsterTemplate {
    /// Check if this monster has the given attribute.
    ///
    /// Attributes are derived from bit fields (`cmove`, `cdefense`) and
    /// boolean fields (`multiplies`, `can_move`).
    ///
    /// Bit masks match `monster_template.c:monster_template_has_attribute()`.
    pub fn has_attribute(&self, attr: MonsterAttribute) -> bool {
        match attr {
            // Boolean fields
            MonsterAttribute::MoveOnlyToAttack => !self.can_move,
            MonsterAttribute::Multiplies => self.multiplies,

            // cmove bit field
            MonsterAttribute::RandomMovement20pc => (self.cmove & 0x00000002) != 0,
            MonsterAttribute::RandomMovement40pc => (self.cmove & 0x00000004) != 0,
            MonsterAttribute::RandomMovement75pc => (self.cmove & 0x00000008) != 0,
            MonsterAttribute::WaterBased => (self.cmove & 0x00000010) != 0,
            MonsterAttribute::LandBased => (self.cmove & 0x00000010) == 0,
            MonsterAttribute::DiesInWrongElement => (self.cmove & 0x00000040) != 0,
            MonsterAttribute::GoodMonster => (self.cmove & 0x00004000) != 0,
            MonsterAttribute::Unspawnable => (self.cmove & 0x00008000) != 0,
            MonsterAttribute::InvisibleMovement => (self.cmove & 0x00010000) != 0,
            MonsterAttribute::MovesThroughDoor => (self.cmove & 0x00020000) != 0,
            MonsterAttribute::MovesThroughWall => (self.cmove & 0x00040000) != 0,
            MonsterAttribute::MovesThroughCreatures => (self.cmove & 0x00080000) != 0,
            MonsterAttribute::PicksUpObjects => (self.cmove & 0x00100000) != 0,
            MonsterAttribute::AnchorsInWater => (self.cmove & 0x00400000) != 0,
            MonsterAttribute::Flying => (self.cmove & 0x00800000) != 0,
            MonsterAttribute::CarriesObjects => (self.cmove & 0x01000000) != 0,
            MonsterAttribute::CarriesGold => (self.cmove & 0x02000000) != 0,
            MonsterAttribute::Carries60pc => (self.cmove & 0x04000000) != 0,
            MonsterAttribute::Carries90pc => (self.cmove & 0x08000000) != 0,
            MonsterAttribute::Carries1d2Things => (self.cmove & 0x10000000) != 0,
            MonsterAttribute::Carries2d2Things => (self.cmove & 0x20000000) != 0,
            MonsterAttribute::Carries4d2Things => (self.cmove & 0x40000000) != 0,
            MonsterAttribute::WinsTheGame => (self.cmove & 0x80000000) != 0,

            // cdefense bit field
            MonsterAttribute::Dragon => (self.cdefense & 0x0001) != 0,
            MonsterAttribute::Monster => (self.cdefense & 0x0002) != 0,
            MonsterAttribute::Evil => (self.cdefense & 0x0004) != 0,
            MonsterAttribute::Undead => (self.cdefense & 0x0008) != 0,
            MonsterAttribute::Demon => (self.cdefense & 0x0400) != 0,
            MonsterAttribute::VulnerableToFrost => (self.cdefense & 0x0010) != 0,
            MonsterAttribute::VulnerableToFire => (self.cdefense & 0x0020) != 0,
            MonsterAttribute::VulnerableToPoison => (self.cdefense & 0x0040) != 0,
            MonsterAttribute::VulnerableToAcid => (self.cdefense & 0x0080) != 0,
            MonsterAttribute::VulnerableToLightning => (self.cdefense & 0x0100) != 0,
            MonsterAttribute::VulnerableToStoneToMud => (self.cdefense & 0x0200) != 0,
            MonsterAttribute::Uncharmable => (self.cdefense & 0x1000) != 0,
            MonsterAttribute::VisibleWithInfravision => (self.cdefense & 0x2000) != 0,
            MonsterAttribute::MaxHitPoints => (self.cdefense & 0x4000) != 0,
            MonsterAttribute::Regenerates => (self.cdefense & 0x8000) != 0,

            // Compound attributes
            MonsterAttribute::SurvivesInWater => {
                self.has_attribute(MonsterAttribute::WaterBased)
                    || !self.has_attribute(MonsterAttribute::DiesInWrongElement)
                    || self.has_attribute(MonsterAttribute::Flying)
            }
            MonsterAttribute::SurvivesOnLand => {
                self.has_attribute(MonsterAttribute::LandBased)
                    || !self.has_attribute(MonsterAttribute::DiesInWrongElement)
                    || self.has_attribute(MonsterAttribute::Flying)
            }
        }
    }
}
