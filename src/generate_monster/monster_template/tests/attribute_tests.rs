//! Tests for MonsterAttribute enum and has_attribute() method.

use super::super::{MonsterAttribute, MonsterTemplate};

/// Verify the enum has the expected number of variants.
/// This matches the C `monster_attribute` enum which has 42 variants.
#[test]
fn attribute_enum_has_all_variants() {
    // The C enum has these variants (from monster_template.h):
    // ma_move_only_to_attack through ma_regenerates

    // We test by checking specific variants exist and have correct discriminants.
    // This will fail until we implement all variants.

    assert_eq!(MonsterAttribute::MoveOnlyToAttack as u32, 0);
    assert_eq!(MonsterAttribute::RandomMovement20pc as u32, 1);
    assert_eq!(MonsterAttribute::RandomMovement40pc as u32, 2);
    assert_eq!(MonsterAttribute::RandomMovement75pc as u32, 3);
    assert_eq!(MonsterAttribute::WaterBased as u32, 4);
    assert_eq!(MonsterAttribute::LandBased as u32, 5);
    assert_eq!(MonsterAttribute::DiesInWrongElement as u32, 6);
    assert_eq!(MonsterAttribute::SurvivesInWater as u32, 7);
    assert_eq!(MonsterAttribute::SurvivesOnLand as u32, 8);
    assert_eq!(MonsterAttribute::GoodMonster as u32, 9);
    assert_eq!(MonsterAttribute::Unspawnable as u32, 10);
    assert_eq!(MonsterAttribute::InvisibleMovement as u32, 11);
    assert_eq!(MonsterAttribute::MovesThroughDoor as u32, 12);
    assert_eq!(MonsterAttribute::MovesThroughWall as u32, 13);
    assert_eq!(MonsterAttribute::MovesThroughCreatures as u32, 14);
    assert_eq!(MonsterAttribute::PicksUpObjects as u32, 15);
    assert_eq!(MonsterAttribute::Multiplies as u32, 16);
    assert_eq!(MonsterAttribute::AnchorsInWater as u32, 17);
    assert_eq!(MonsterAttribute::Flying as u32, 18);
    assert_eq!(MonsterAttribute::CarriesObjects as u32, 19);
    assert_eq!(MonsterAttribute::CarriesGold as u32, 20);
    assert_eq!(MonsterAttribute::Carries60pc as u32, 21);
    assert_eq!(MonsterAttribute::Carries90pc as u32, 22);
    assert_eq!(MonsterAttribute::Carries1d2Things as u32, 23);
    assert_eq!(MonsterAttribute::Carries2d2Things as u32, 24);
    assert_eq!(MonsterAttribute::Carries4d2Things as u32, 25);
    assert_eq!(MonsterAttribute::WinsTheGame as u32, 26);
    assert_eq!(MonsterAttribute::Dragon as u32, 27);
    assert_eq!(MonsterAttribute::Monster as u32, 28);
    assert_eq!(MonsterAttribute::Evil as u32, 29);
    assert_eq!(MonsterAttribute::Undead as u32, 30);
    assert_eq!(MonsterAttribute::Demon as u32, 31);
    assert_eq!(MonsterAttribute::VulnerableToFrost as u32, 32);
    assert_eq!(MonsterAttribute::VulnerableToFire as u32, 33);
    assert_eq!(MonsterAttribute::VulnerableToPoison as u32, 34);
    assert_eq!(MonsterAttribute::VulnerableToAcid as u32, 35);
    assert_eq!(MonsterAttribute::VulnerableToLightning as u32, 36);
    assert_eq!(MonsterAttribute::VulnerableToStoneToMud as u32, 37);
    assert_eq!(MonsterAttribute::Uncharmable as u32, 38);
    assert_eq!(MonsterAttribute::VisibleWithInfravision as u32, 39);
    assert_eq!(MonsterAttribute::MaxHitPoints as u32, 40);
    assert_eq!(MonsterAttribute::Regenerates as u32, 41);
}

// ============================================================================
// Test fixtures
// ============================================================================

/// Create a minimal template for testing specific attributes.
fn make_template(cmove: u64, cdefense: u64, multiplies: bool, can_move: bool) -> MonsterTemplate {
    MonsterTemplate {
        area_effect_radius: 0,
        ac: 0,
        name: "Test Monster",
        cmove,
        spells: 0,
        cdefense,
        sleep: 0,
        mexp: 0,
        speed: 0,
        symbol: 'T',
        hit_die: "1d1",
        damage: "1 1 1d1",
        level: 0,
        magic_resistance: 0,
        multiplies,
        can_move,
    }
}

// ============================================================================
// Tests for attributes from `can_move` field
// ============================================================================

#[test]
fn has_attribute_move_only_to_attack_when_cannot_move() {
    let template = make_template(0, 0, false, false); // can_move = false
    assert!(template.has_attribute(MonsterAttribute::MoveOnlyToAttack));
}

#[test]
fn has_attribute_move_only_to_attack_false_when_can_move() {
    let template = make_template(0, 0, false, true); // can_move = true
    assert!(!template.has_attribute(MonsterAttribute::MoveOnlyToAttack));
}

// ============================================================================
// Tests for attributes from `multiplies` field
// ============================================================================

#[test]
fn has_attribute_multiplies_when_true() {
    let template = make_template(0, 0, true, true); // multiplies = true
    assert!(template.has_attribute(MonsterAttribute::Multiplies));
}

#[test]
fn has_attribute_multiplies_false_when_false() {
    let template = make_template(0, 0, false, true); // multiplies = false
    assert!(!template.has_attribute(MonsterAttribute::Multiplies));
}

// ============================================================================
// Tests for attributes from `cmove` bit field
// ============================================================================

#[test]
fn has_attribute_random_movement_20pc() {
    let template = make_template(0x00000002, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::RandomMovement20pc));
}

#[test]
fn has_attribute_random_movement_40pc() {
    let template = make_template(0x00000004, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::RandomMovement40pc));
}

#[test]
fn has_attribute_random_movement_75pc() {
    let template = make_template(0x00000008, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::RandomMovement75pc));
}

#[test]
fn has_attribute_water_based() {
    let template = make_template(0x00000010, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::WaterBased));
    assert!(!template.has_attribute(MonsterAttribute::LandBased)); // mutually exclusive
}

#[test]
fn has_attribute_land_based() {
    let template = make_template(0, 0, false, true); // no water bit = land based
    assert!(template.has_attribute(MonsterAttribute::LandBased));
    assert!(!template.has_attribute(MonsterAttribute::WaterBased));
}

#[test]
fn has_attribute_dies_in_wrong_element() {
    let template = make_template(0x00000040, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::DiesInWrongElement));
}

#[test]
fn has_attribute_good_monster() {
    let template = make_template(0x00004000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::GoodMonster));
}

#[test]
fn has_attribute_unspawnable() {
    let template = make_template(0x00008000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Unspawnable));
}

#[test]
fn has_attribute_invisible_movement() {
    let template = make_template(0x00010000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::InvisibleMovement));
}

#[test]
fn has_attribute_moves_through_door() {
    let template = make_template(0x00020000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::MovesThroughDoor));
}

#[test]
fn has_attribute_moves_through_wall() {
    let template = make_template(0x00040000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::MovesThroughWall));
}

#[test]
fn has_attribute_moves_through_creatures() {
    let template = make_template(0x00080000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::MovesThroughCreatures));
}

#[test]
fn has_attribute_picks_up_objects() {
    let template = make_template(0x00100000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::PicksUpObjects));
}

#[test]
fn has_attribute_anchors_in_water() {
    let template = make_template(0x00400000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::AnchorsInWater));
}

#[test]
fn has_attribute_flying() {
    let template = make_template(0x00800000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Flying));
}

#[test]
fn has_attribute_carries_objects() {
    let template = make_template(0x01000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::CarriesObjects));
}

#[test]
fn has_attribute_carries_gold() {
    let template = make_template(0x02000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::CarriesGold));
}

#[test]
fn has_attribute_carries_60pc() {
    let template = make_template(0x04000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Carries60pc));
}

#[test]
fn has_attribute_carries_90pc() {
    let template = make_template(0x08000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Carries90pc));
}

#[test]
fn has_attribute_carries_1d2_things() {
    let template = make_template(0x10000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Carries1d2Things));
}

#[test]
fn has_attribute_carries_2d2_things() {
    let template = make_template(0x20000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Carries2d2Things));
}

#[test]
fn has_attribute_carries_4d2_things() {
    let template = make_template(0x40000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::Carries4d2Things));
}

#[test]
fn has_attribute_wins_the_game() {
    let template = make_template(0x80000000, 0, false, true);
    assert!(template.has_attribute(MonsterAttribute::WinsTheGame));
}

// ============================================================================
// Tests for attributes from `cdefense` bit field
// ============================================================================

#[test]
fn has_attribute_dragon() {
    let template = make_template(0, 0x0001, false, true);
    assert!(template.has_attribute(MonsterAttribute::Dragon));
}

#[test]
fn has_attribute_monster() {
    let template = make_template(0, 0x0002, false, true);
    assert!(template.has_attribute(MonsterAttribute::Monster));
}

#[test]
fn has_attribute_evil() {
    let template = make_template(0, 0x0004, false, true);
    assert!(template.has_attribute(MonsterAttribute::Evil));
}

#[test]
fn has_attribute_undead() {
    let template = make_template(0, 0x0008, false, true);
    assert!(template.has_attribute(MonsterAttribute::Undead));
}

#[test]
fn has_attribute_demon() {
    let template = make_template(0, 0x0400, false, true);
    assert!(template.has_attribute(MonsterAttribute::Demon));
}

#[test]
fn has_attribute_vulnerable_to_frost() {
    let template = make_template(0, 0x0010, false, true);
    assert!(template.has_attribute(MonsterAttribute::VulnerableToFrost));
}

#[test]
fn has_attribute_vulnerable_to_fire() {
    let template = make_template(0, 0x0020, false, true);
    assert!(template.has_attribute(MonsterAttribute::VulnerableToFire));
}

#[test]
fn has_attribute_vulnerable_to_poison() {
    let template = make_template(0, 0x0040, false, true);
    assert!(template.has_attribute(MonsterAttribute::VulnerableToPoison));
}

#[test]
fn has_attribute_vulnerable_to_acid() {
    let template = make_template(0, 0x0080, false, true);
    assert!(template.has_attribute(MonsterAttribute::VulnerableToAcid));
}

#[test]
fn has_attribute_vulnerable_to_lightning() {
    let template = make_template(0, 0x0100, false, true);
    assert!(template.has_attribute(MonsterAttribute::VulnerableToLightning));
}

#[test]
fn has_attribute_vulnerable_to_stone_to_mud() {
    let template = make_template(0, 0x0200, false, true);
    assert!(template.has_attribute(MonsterAttribute::VulnerableToStoneToMud));
}

#[test]
fn has_attribute_uncharmable() {
    let template = make_template(0, 0x1000, false, true);
    assert!(template.has_attribute(MonsterAttribute::Uncharmable));
}

#[test]
fn has_attribute_visible_with_infravision() {
    let template = make_template(0, 0x2000, false, true);
    assert!(template.has_attribute(MonsterAttribute::VisibleWithInfravision));
}

#[test]
fn has_attribute_max_hit_points() {
    let template = make_template(0, 0x4000, false, true);
    assert!(template.has_attribute(MonsterAttribute::MaxHitPoints));
}

#[test]
fn has_attribute_regenerates() {
    let template = make_template(0, 0x8000, false, true);
    assert!(template.has_attribute(MonsterAttribute::Regenerates));
}

// ============================================================================
// Tests for compound attributes (depend on multiple flags)
// ============================================================================

#[test]
fn has_attribute_survives_in_water_when_water_based() {
    // Water-based creature survives in water
    let template = make_template(0x00000010, 0, false, true); // water_based bit
    assert!(template.has_attribute(MonsterAttribute::SurvivesInWater));
}

#[test]
fn has_attribute_survives_in_water_when_not_dies_in_wrong_element() {
    // Land-based but doesn't die in wrong element = survives in water
    let template = make_template(0, 0, false, true); // no dies_in_wrong_element bit
    assert!(template.has_attribute(MonsterAttribute::SurvivesInWater));
}

#[test]
fn has_attribute_survives_in_water_when_flying() {
    // Land-based, dies in wrong element, but flying = survives in water
    let template = make_template(0x00800040, 0, false, true); // flying + dies_in_wrong
    assert!(template.has_attribute(MonsterAttribute::SurvivesInWater));
}

#[test]
fn has_attribute_survives_in_water_false_when_land_based_and_dies() {
    // Land-based, dies in wrong element, not flying = doesn't survive in water
    let template = make_template(0x00000040, 0, false, true); // dies_in_wrong only
    assert!(!template.has_attribute(MonsterAttribute::SurvivesInWater));
}

#[test]
fn has_attribute_survives_on_land_when_land_based() {
    // Land-based creature survives on land
    let template = make_template(0, 0, false, true); // no water_based bit
    assert!(template.has_attribute(MonsterAttribute::SurvivesOnLand));
}

#[test]
fn has_attribute_survives_on_land_when_not_dies_in_wrong_element() {
    // Water-based but doesn't die in wrong element = survives on land
    let template = make_template(0x00000010, 0, false, true); // water_based, no dies
    assert!(template.has_attribute(MonsterAttribute::SurvivesOnLand));
}

#[test]
fn has_attribute_survives_on_land_when_flying() {
    // Water-based, dies in wrong element, but flying = survives on land
    let template = make_template(0x00800050, 0, false, true); // water + flying + dies
    assert!(template.has_attribute(MonsterAttribute::SurvivesOnLand));
}

#[test]
fn has_attribute_survives_on_land_false_when_water_based_and_dies() {
    // Water-based, dies in wrong element, not flying = doesn't survive on land
    let template = make_template(0x00000050, 0, false, true); // water + dies
    assert!(!template.has_attribute(MonsterAttribute::SurvivesOnLand));
}
