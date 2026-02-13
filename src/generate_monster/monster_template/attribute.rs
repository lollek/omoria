//! Monster attribute enum.
//!
//! Maps to bit fields in `cmove`, `cdefense`, and the `attributes` struct
//! from the C `monster_template_t`.

/// Monster attribute flags.
///
/// These correspond to the `monster_attribute` enum in `monster_template.h`.
/// Each attribute maps to either:
/// - A bit in `cmove` (movement/behavior flags)
/// - A bit in `cdefense` (defense/vulnerability flags)
/// - A field in `monster_attributes` struct (`multiplies`, `can_move`)
/// - A compound check (e.g., `SurvivesInWater` depends on multiple flags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum MonsterAttribute {
    /// Monster only moves to attack (no random movement).
    MoveOnlyToAttack = 0,
    /// 20% chance of random movement.
    RandomMovement20pc = 1,
    /// 40% chance of random movement.
    RandomMovement40pc = 2,
    /// 75% chance of random movement.
    RandomMovement75pc = 3,
    /// Monster is water-based (lives in water).
    WaterBased = 4,
    /// Monster is land-based (lives on land).
    LandBased = 5,
    /// Monster dies if in wrong element (water/land).
    DiesInWrongElement = 6,
    /// Monster can survive in water (compound: water-based OR !dies-in-wrong OR flying).
    SurvivesInWater = 7,
    /// Monster can survive on land (compound: land-based OR !dies-in-wrong OR flying).
    SurvivesOnLand = 8,
    /// Killing this monster affects reputation negatively.
    GoodMonster = 9,
    /// Cannot be spawned by normal means.
    Unspawnable = 10,
    /// Monster moves invisibly.
    InvisibleMovement = 11,
    /// Monster can move through doors.
    MovesThroughDoor = 12,
    /// Monster can move through walls.
    MovesThroughWall = 13,
    /// Monster can move through other creatures.
    MovesThroughCreatures = 14,
    /// Monster picks up objects.
    PicksUpObjects = 15,
    /// Monster can multiply/reproduce.
    Multiplies = 16,
    /// Monster anchors in water (doesn't drift).
    AnchorsInWater = 17,
    /// Monster can fly.
    Flying = 18,
    /// Monster carries objects.
    CarriesObjects = 19,
    /// Monster carries gold.
    CarriesGold = 20,
    /// 60% chance monster carries something.
    Carries60pc = 21,
    /// 90% chance monster carries something.
    Carries90pc = 22,
    /// Monster carries 1d2 things.
    Carries1d2Things = 23,
    /// Monster carries 2d2 things.
    Carries2d2Things = 24,
    /// Monster carries 4d2 things.
    Carries4d2Things = 25,
    /// Killing this monster wins the game.
    WinsTheGame = 26,
    /// Monster is a dragon (hurt by Slay Dragon).
    Dragon = 27,
    /// Monster type (hurt by Slay Monster).
    Monster = 28,
    /// Monster is evil (hurt by Slay Evil).
    Evil = 29,
    /// Monster is undead (hurt by Slay Undead).
    Undead = 30,
    /// Monster is a demon (hurt by Slay Demon).
    Demon = 31,
    /// Monster is vulnerable to frost.
    VulnerableToFrost = 32,
    /// Monster is vulnerable to fire.
    VulnerableToFire = 33,
    /// Monster is vulnerable to poison.
    VulnerableToPoison = 34,
    /// Monster is vulnerable to acid.
    VulnerableToAcid = 35,
    /// Monster is vulnerable to lightning.
    VulnerableToLightning = 36,
    /// Monster is vulnerable to stone-to-mud.
    VulnerableToStoneToMud = 37,
    /// Monster cannot be charmed or put to sleep.
    Uncharmable = 38,
    /// Monster is visible with infravision.
    VisibleWithInfravision = 39,
    /// Monster always has maximum hit points.
    MaxHitPoints = 40,
    /// Monster regenerates hit points.
    Regenerates = 41,
}
