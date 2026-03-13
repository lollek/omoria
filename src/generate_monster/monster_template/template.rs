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
    /// Max range that creature is able to "notice" the player.
    pub area_effect_radius: u8,
    /// Armor class.
    pub ac: u8,
    /// Monster name (e.g., "Kobold", "Balrog").
    pub name: &'static str,
    /// Movement/behavior bit field.
	///	00000001	Move only to attack
	///	00000002	20% random movement
	///	00000004	40% random movement
	///	00000008	75% random movement
	///	00000010	On = Water-based; Off = Land-based
	///	00000040	Drowns/Suffocates in wrong element
	///	00000300	monster rate in wrong elm't (0=stop,3=full)
	/// Special	
    /// +	00004000	Is a 'good' monster (reputation)
	///	+	00008000	Is a monster that will not normally appear
	///					(such as Town Guards)
	///					These can only be summoned by summon
	///					monster by name.
	///	+	00010000	Invisible movement
	///	+	00020000	Move through door
	///	+	00040000	Move through wall
	///	+	00080000	Move through creatures
	///	+	00100000	Picks up objects
	///	+	00200000	Multiply monster
	///	+	00400000	Can anchor in water
	///	+	00800000	Flying creature
	/// Carries =	01000000	Carries objects.
	///	=	02000000	Carries gold.
	///	=	04000000	Has 60% of time.
	///	=	08000000	Has 90% of time.
	///	=	10000000	1d2 objects/gold.
	///	=	20000000	2d2 objects/gold.
	///	=	40000000	4d2 objects/gold.
	/// Special ~	80000000	Win-the-Game creature.
    pub cmove: u64,
    /// Spell bit field.
    /// Frequency	00000001    1	  These add up to x.  Then
    /// (1 in x).	00000002    2	  if RANDINT(X) = 1 the
	///	00000004    4	  creature casts a spell.
	///	00000008    8
    /// Spells	=	00000010  Teleport short (blink)
	/// =	00000020  Teleport long
	/// =	00000040  Teleport player to monster
	/// =	00000080  Cause light wound
	/// =	00000100  Cause serious wound
	/// =	00000200  Hold person (Paralysis)
	/// =	00000400  Cause blindness
	/// =	00000800  Cause confusion
	/// =	00001000  Cause fear
	/// =	00002000  Summon monster
	/// =	00004000  Summon undead
	/// =	00008000  Slow Person
	/// =	00010000  Drain Mana
	/// =	00020000  Shadow Breath/Orb of Draining
	/// =	00040000  Not Used
	/// Breaths +	00080000  Breath Lightning
	/// +	00100000  Breath Gas
	/// +	00200000  Breath Acid
	/// +	00400000  Breath Frost
	/// +	00800000  Breath Fire
	/// =	01000000  Casts Illusion
	/// =	02000000  Summon Demon
	/// =	04000000  Summon Multiplying Monster (heh heh)
	/// =	08000000  Gaze from distance for petrification
	/// .	80000000  makes no casting 1 in x (instead of casting 1 in x)
    pub spells: u64,
    /// Defense/vulnerability bit field.
	///	0001	Hurt by Slay Dragon.
	///	0002	Hurt by Slay Monster.
	///	0004	Hurt by Slay Evil.
	///	0008	Hurt by Slay Undead.
	///	0010	Hurt by Frost.
	///	0020	Hurt by Fire.
	///	0040	Hurt by Poison.
	///	0080	Hurt by Acid.
	///	0100	Hurt by Light-Wand.
	///	0200	Hurt by Stone-to-Mud.
	///	0400	Hurt by Slay Demon.
	///	0800	Not used.
	///	1000	Cannot be charmed or slept.
	///	2000	Can be seen with infra-vision.
	///	4000	Max Hit points.
	///	8000	Regenerates.
    pub cdefense: u64,
    /// Inactive counter when spawned.
    /// A measure in turns of how fast creature will notice player (on the average).
    pub sleep: i16,
    /// Experience for slaying the creature.
    pub mexp: i64,
    /// Movement speed.
    pub speed: i8,
    /// Symbol displayed on map.
    pub symbol: char,
    /// Hit dice (e.g., "20d8").
    pub hit_die: &'static str,
    /// Attack damage string (e.g., "1 1 3d3|1 1 4d4").
    /// 
    /// Attack types:
	///	1	Normal attack
	///	2	Lose Strength
	///	3	Confusion attack
	///	4	Fear attack
	///	5	Fire attack
	///	6	Acid attack
	///	7	Cold attack
	///	8	Lightning attack
	///	9	Corrosion attack
	///	10	Blindness attack
	///	11	Paralysis attack
	///	12	Steal Money
	///	13	Steal Object
	///	14	Poison
	///	15	Lose dexterity
	///	16	Lose constitution
	///	17	Lose intelligence
	///	18	Lose wisdom
	///	19	Lose experience
	///	20	Aggravation
	///	21	Disenchants
	///	22	Eats food
	///	23	Eats light
	///	24	Eats charges
	///	25	Lose charisma
	///	26	Petrification
	///	27	POISON poison
	///	99	Blank
    /// 
    /// 
	///	Attack descriptions:
	///	1	hits you.
	///	2	bites you.
	///	3	claws you.
	///	4	stings you.
	///	5	touches you.
	///	6	kicks you.
	///	7	gazes at you.
	///	8	breathes on you.
	///	9	spits on you.
	///	10	makes a horrible wail.
	///	11	embraces you.
	///	12	crawls on you.
	///	13	releases a cloud of spores.
	///	14	begs you for money.
	///	15	You've been slimed.
	///	16	crushes you.
	///	17	tramples you.
	///	18	drools on you.
	///	19	insults you.
	///	23	plays a song.
	///	24	kisses you.
	///	25	gores you.
	///	26	"bovine"s you.
	///	27	electrocutes you.
	///	28	inks you.
	///	29	entangles you.
	///	30	blood sucks you.
	///	31	goes for your throat.
	///	32	blows bubbles at you.
	///	33	squawks at you.
	///	34	pecks at you.
	///	35	barks at you.
	///	36	rubs against your leg.
	///	99	is repelled.
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
