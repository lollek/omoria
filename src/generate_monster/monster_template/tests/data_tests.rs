//! Tests for monster template data correctness.
//!
//! These tests verify the Rust `MONSTER_TEMPLATES` array matches
//! the C `monster_templates[]` array from `monster_template.c`.

use super::super::{MonsterAttribute, MONSTER_TEMPLATES};

// ============================================================================
// Array size tests
// ============================================================================

/// The C array has 392 entries (monster_template_size).
/// This must match exactly for interop to work.
#[test]
fn monster_templates_count_matches_c() {
    assert_eq!(MONSTER_TEMPLATES.len(), 392);
}

// ============================================================================
// Spot-check specific monsters
// ============================================================================

/// Index 0: Placeholder monster (level 0)
#[test]
fn monster_template_index_0_placeholder() {
    let m = &MONSTER_TEMPLATES[0];
    assert_eq!(m.name, "<<Placeholder>>");
    assert_eq!(m.level, 0);
    assert_eq!(m.symbol, 'p');
    assert_eq!(m.area_effect_radius, 10);
    assert_eq!(m.ac, 1);
    assert_eq!(m.mexp, 50);
    assert_eq!(m.magic_resistance, 20);
}

/// Index 1: Town Wizard (level 0)
#[test]
fn monster_template_index_1_town_wizard() {
    let m = &MONSTER_TEMPLATES[1];
    assert_eq!(m.name, "Town Wizard");
    assert_eq!(m.level, 0);
    assert_eq!(m.symbol, 'p');
}

/// Index 2: Town Guard (level 0)
#[test]
fn monster_template_index_2_town_guard() {
    let m = &MONSTER_TEMPLATES[2];
    assert_eq!(m.name, "Town Guard");
    assert_eq!(m.level, 0);
    assert_eq!(m.symbol, 'p');
    assert_eq!(m.mexp, 25);
    assert_eq!(m.damage, "1 1 4d4|1 1 4d4");
}

/// Index 19: Brown Imp - first level 1 monster, first that multiplies
#[test]
fn monster_template_index_19_brown_imp() {
    let m = &MONSTER_TEMPLATES[19];
    assert_eq!(m.name, "Brown Imp");
    assert_eq!(m.level, 1);
    assert_eq!(m.symbol, 'i');
    assert!(m.multiplies);
    assert!(m.can_move);
}

/// Index 21: Grey Mushroom patch - stationary monster (can_move = false)
#[test]
fn monster_template_index_21_grey_mushroom() {
    let m = &MONSTER_TEMPLATES[21];
    assert_eq!(m.name, "Grey Mushroom patch");
    assert_eq!(m.level, 1);
    assert_eq!(m.symbol, ',');
    assert!(!m.can_move);
    assert!(m.has_attribute(MonsterAttribute::MoveOnlyToAttack));
}

/// Index 27: Kobold - classic early game monster
#[test]
fn monster_template_index_27_kobold() {
    let m = &MONSTER_TEMPLATES[27];
    assert_eq!(m.name, "Kobold");
    assert_eq!(m.level, 1);
    assert_eq!(m.symbol, 'k');
    assert_eq!(m.hit_die, "3d7");
}

/// Last monster: Balrog (index 391, level 100)
#[test]
fn monster_template_last_is_balrog() {
    let m = &MONSTER_TEMPLATES[391];
    assert_eq!(m.name, "Balrog");
    assert_eq!(m.level, 100);
    assert_eq!(m.symbol, 'B');
    assert_eq!(m.ac, 125);
    assert_eq!(m.mexp, 55000);
    assert_eq!(m.magic_resistance, 255);
    assert_eq!(m.cmove, 0xFF1F0300);
    assert_eq!(m.spells, 0x0281C743);
    assert_eq!(m.cdefense, 0x5404);
    // Balrog should be evil and a demon
    assert!(m.has_attribute(MonsterAttribute::Evil));
    assert!(m.has_attribute(MonsterAttribute::Demon));
}

/// Second to last: Evil Iggy (index 390, level 100)
#[test]
fn monster_template_index_299_evil_iggy() {
    let m = &MONSTER_TEMPLATES[390];
    assert_eq!(m.name, "Evil Iggy");
    assert_eq!(m.level, 100);
    assert_eq!(m.magic_resistance, 175);
}

/// Emperor Faerie Dragon (index 389, level 100)
#[test]
fn monster_template_index_298_emperor_faerie_dragon() {
    let m = &MONSTER_TEMPLATES[389];
    assert_eq!(m.name, "Emperor Faerie Dragon");
    assert_eq!(m.level, 100);
    assert_eq!(m.symbol, 'F');
}

// ============================================================================
// Level distribution tests
// ============================================================================

/// All monsters should have level between 0 and 100
#[test]
fn monster_template_levels_in_valid_range() {
    for (i, m) in MONSTER_TEMPLATES.iter().enumerate() {
        assert!(
            m.level >= 0 && m.level <= 100,
            "Monster {} '{}' has invalid level {}",
            i,
            m.name,
            m.level
        );
    }
}

/// Level 0 monsters exist (town creatures)
#[test]
fn monster_template_has_level_0_monsters() {
    let count = MONSTER_TEMPLATES.iter().filter(|m| m.level == 0).count();
    assert!(count > 0, "Expected some level 0 monsters");
    // C has 19 level 0 monsters (indices 0-18)
    assert_eq!(count, 19);
}

/// Level 1 monsters exist
#[test]
fn monster_template_has_level_1_monsters() {
    let count = MONSTER_TEMPLATES.iter().filter(|m| m.level == 1).count();
    assert!(count > 0, "Expected some level 1 monsters");
}

/// Level 100 monsters exist (endgame bosses)
#[test]
fn monster_template_has_level_100_monsters() {
    let count = MONSTER_TEMPLATES.iter().filter(|m| m.level == 100).count();
    assert!(count > 0, "Expected some level 100 monsters");
    // C has 3 level 100 monsters: Emperor Faerie Dragon, Evil Iggy, Balrog
    assert_eq!(count, 3);
}

// ============================================================================
// Bit field consistency tests
// ============================================================================

/// Flying monsters should have the flying bit set in cmove
#[test]
fn monster_template_flying_bit_consistency() {
    for (i, m) in MONSTER_TEMPLATES.iter().enumerate() {
        let has_flying_bit = (m.cmove & 0x00800000) != 0;
        let has_flying_attr = m.has_attribute(MonsterAttribute::Flying);
        assert_eq!(
            has_flying_bit, has_flying_attr,
            "Monster {} '{}' flying bit mismatch",
            i, m.name
        );
    }
}

/// Evil monsters should have the evil bit set in cdefense
#[test]
fn monster_template_evil_bit_consistency() {
    for (i, m) in MONSTER_TEMPLATES.iter().enumerate() {
        let has_evil_bit = (m.cdefense & 0x0004) != 0;
        let has_evil_attr = m.has_attribute(MonsterAttribute::Evil);
        assert_eq!(
            has_evil_bit, has_evil_attr,
            "Monster {} '{}' evil bit mismatch",
            i, m.name
        );
    }
}
