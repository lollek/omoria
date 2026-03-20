//! C ABI wrappers for monster template interop.
//!
//! This module provides C-compatible accessor functions that let C code read
//! from the Rust `MONSTER_TEMPLATES` data without touching the array directly.
//!
//! String getters (`get_name`, `get_hit_die`, `get_damage`) return pointers to
//! a pre-computed `CStringCache`.  Scalar getters read `MONSTER_TEMPLATES`
//! directly.  Out-of-bounds access returns the "Glitch" fallback.

use super::{MonsterAttribute, MonsterTemplate};

use std::sync::LazyLock;

// =============================================================================
// String cache
// =============================================================================

/// Copies a `&str` into a fixed-size `[c_char; N]` with a NUL terminator.
macro_rules! str_to_c_array {
    ($s:expr, $n:literal) => {{
        let mut arr = [0i8; $n];
        let bytes = $s.as_bytes();
        let len = if bytes.len() < ($n - 1) {
            bytes.len()
        } else {
            $n - 1
        };
        let mut i = 0;
        while i < len {
            arr[i] = bytes[i] as i8;
            i += 1;
        }
        arr
    }};
}

/// Pre-computed NUL-terminated C strings for the three string fields that
/// need to be returned as `*const c_char` pointers.
#[derive(Debug, Clone, Copy)]
struct CStringCache {
    name: [libc::c_char; 28],
    hit_die: [libc::c_char; 7],
    damage: [libc::c_char; 36],
}

impl CStringCache {
    const fn from_template(t: &MonsterTemplate) -> Self {
        Self {
            name: str_to_c_array!(t.name, 28),
            hit_die: str_to_c_array!(t.hit_die, 7),
            damage: str_to_c_array!(t.damage, 36),
        }
    }
}

/// Pre-computed string cache, one entry per monster template.
static STRING_CACHE: LazyLock<Vec<CStringCache>> = LazyLock::new(|| {
    super::MONSTER_TEMPLATES
        .iter()
        .map(|t| CStringCache::from_template(t))
        .collect()
});

// =============================================================================
// Glitch fallback
// =============================================================================

/// Fallback template for out-of-bounds access.
/// A visible "Glitch" monster makes indexing bugs obvious in-game.
static GLITCH_TEMPLATE: MonsterTemplate = MonsterTemplate {
    area_effect_radius: 0,
    ac: 0,
    name: "Glitch",
    cmove: 0,
    spells: 0,
    cdefense: 0,
    sleep: 0,
    mexp: 0,
    speed: 0,
    symbol: '?',
    hit_die: "1d1",
    damage: "",
    level: 0,
    magic_resistance: 0,
    multiplies: false,
    can_move: false,
};

/// Fallback string cache for out-of-bounds access.
static GLITCH_STRING_CACHE: LazyLock<CStringCache> =
    LazyLock::new(|| CStringCache::from_template(&GLITCH_TEMPLATE));

// =============================================================================
// Helpers
// =============================================================================

/// Get the Rust template at `index`, or the Glitch fallback for OOB.
fn get_template(index: libc::c_long) -> &'static MonsterTemplate {
    super::MONSTER_TEMPLATES
        .get(index as usize)
        .unwrap_or(&GLITCH_TEMPLATE)
}

/// Get the string cache at `index`, or the Glitch fallback for OOB.
fn get_string_cache(index: libc::c_long) -> &'static CStringCache {
    STRING_CACHE
        .get(index as usize)
        .unwrap_or(&GLITCH_STRING_CACHE)
}

/// Convert C monster_attribute enum value to Rust MonsterAttribute.
fn monster_attribute_from_c(attr: libc::c_int) -> Option<MonsterAttribute> {
    match attr {
        0 => Some(MonsterAttribute::MoveOnlyToAttack),
        1 => Some(MonsterAttribute::RandomMovement20pc),
        2 => Some(MonsterAttribute::RandomMovement40pc),
        3 => Some(MonsterAttribute::RandomMovement75pc),
        4 => Some(MonsterAttribute::WaterBased),
        5 => Some(MonsterAttribute::LandBased),
        6 => Some(MonsterAttribute::DiesInWrongElement),
        7 => Some(MonsterAttribute::SurvivesInWater),
        8 => Some(MonsterAttribute::SurvivesOnLand),
        9 => Some(MonsterAttribute::GoodMonster),
        10 => Some(MonsterAttribute::Unspawnable),
        11 => Some(MonsterAttribute::InvisibleMovement),
        12 => Some(MonsterAttribute::MovesThroughDoor),
        13 => Some(MonsterAttribute::MovesThroughWall),
        14 => Some(MonsterAttribute::MovesThroughCreatures),
        15 => Some(MonsterAttribute::PicksUpObjects),
        16 => Some(MonsterAttribute::Multiplies),
        17 => Some(MonsterAttribute::AnchorsInWater),
        18 => Some(MonsterAttribute::Flying),
        19 => Some(MonsterAttribute::CarriesObjects),
        20 => Some(MonsterAttribute::CarriesGold),
        21 => Some(MonsterAttribute::Carries60pc),
        22 => Some(MonsterAttribute::Carries90pc),
        23 => Some(MonsterAttribute::Carries1d2Things),
        24 => Some(MonsterAttribute::Carries2d2Things),
        25 => Some(MonsterAttribute::Carries4d2Things),
        26 => Some(MonsterAttribute::WinsTheGame),
        27 => Some(MonsterAttribute::Dragon),
        28 => Some(MonsterAttribute::Monster),
        29 => Some(MonsterAttribute::Evil),
        30 => Some(MonsterAttribute::Undead),
        31 => Some(MonsterAttribute::Demon),
        32 => Some(MonsterAttribute::VulnerableToFrost),
        33 => Some(MonsterAttribute::VulnerableToFire),
        34 => Some(MonsterAttribute::VulnerableToPoison),
        35 => Some(MonsterAttribute::VulnerableToAcid),
        36 => Some(MonsterAttribute::VulnerableToLightning),
        37 => Some(MonsterAttribute::VulnerableToStoneToMud),
        38 => Some(MonsterAttribute::Uncharmable),
        39 => Some(MonsterAttribute::VisibleWithInfravision),
        40 => Some(MonsterAttribute::MaxHitPoints),
        41 => Some(MonsterAttribute::Regenerates),
        _ => None,
    }
}

// =============================================================================
// C ABI exports
// =============================================================================

/// Return the number of monster templates.
#[no_mangle]
pub extern "C" fn monster_template_count() -> libc::c_long {
    super::MONSTER_TEMPLATES.len() as libc::c_long
}

// -- String getters (return pointers from the string cache) -------------------

/// Return the name as a NUL-terminated C string pointer (valid for program lifetime).
#[no_mangle]
pub extern "C" fn monster_template_get_name(index: libc::c_long) -> *const libc::c_char {
    get_string_cache(index).name.as_ptr()
}

/// Return the hit die string as a NUL-terminated C string pointer.
#[no_mangle]
pub extern "C" fn monster_template_get_hit_die(index: libc::c_long) -> *const libc::c_char {
    get_string_cache(index).hit_die.as_ptr()
}

/// Return the damage string as a NUL-terminated C string pointer.
#[no_mangle]
pub extern "C" fn monster_template_get_damage(index: libc::c_long) -> *const libc::c_char {
    get_string_cache(index).damage.as_ptr()
}

// -- Scalar getters (read MONSTER_TEMPLATES directly) -------------------------

/// Return the map symbol of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_symbol(index: libc::c_long) -> libc::c_char {
    get_template(index).symbol as libc::c_char
}

/// Return the level of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_level(index: libc::c_long) -> i8 {
    get_template(index).level
}

/// Return the speed of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_speed(index: libc::c_long) -> i8 {
    get_template(index).speed
}

/// Return the armor class of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_ac(index: libc::c_long) -> u8 {
    get_template(index).ac
}

/// Return the experience value of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_mexp(index: libc::c_long) -> i64 {
    get_template(index).mexp
}

/// Return the sleep/inactive counter of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_sleep(index: libc::c_long) -> i16 {
    get_template(index).sleep
}

/// Return the area effect radius of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_area_effect_radius(index: libc::c_long) -> u8 {
    get_template(index).area_effect_radius
}

/// Return the magic resistance of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_magic_resistance(index: libc::c_long) -> u8 {
    get_template(index).magic_resistance
}

// -- Spell accessors ----------------------------------------------------------

/// Return the raw `spells` bitfield of the monster template at `index`.
#[no_mangle]
pub extern "C" fn monster_template_get_spells_raw(index: libc::c_long) -> u64 {
    get_template(index).spells
}

/// Whether the monster template at `index` has any spells.
#[no_mangle]
pub extern "C" fn monster_template_has_spells(index: libc::c_long) -> bool {
    get_template(index).spells > 0
}

/// Return the spell frequency (bits 0–3 of `spells`).
#[no_mangle]
pub extern "C" fn monster_template_spell_frequency(index: libc::c_long) -> u8 {
    (get_template(index).spells & 0x0000000F) as u8
}

/// Whether the spell frequency is inverted (bit 31 of `spells`).
#[no_mangle]
pub extern "C" fn monster_template_spell_frequency_is_inverted(index: libc::c_long) -> bool {
    (get_template(index).spells & 0x80000000) != 0
}

/// Return the spell choice bits (bits 4–27 of `spells`).
#[no_mangle]
pub extern "C" fn monster_template_spell_choice_bits(index: libc::c_long) -> u32 {
    (get_template(index).spells & 0x0FFFFFF0) as u32
}

// -- Bitfield accessors -------------------------------------------------------

/// Return the raw `cmove` bitfield.
#[no_mangle]
pub extern "C" fn monster_template_get_cmove(index: libc::c_long) -> u64 {
    get_template(index).cmove
}

/// Return the raw `cdefense` bitfield.
#[no_mangle]
pub extern "C" fn monster_template_get_cdefense(index: libc::c_long) -> u64 {
    get_template(index).cdefense
}

/// Return the movement speed (bits 8–9 of `cmove`, divided by 256).
#[no_mangle]
pub extern "C" fn monster_template_movement_speed(index: libc::c_long) -> u8 {
    ((get_template(index).cmove & 0x00000300) / 256) as u8
}

/// Return the swimming level (bits 8–10 of `cmove`, divided by 256).
#[no_mangle]
pub extern "C" fn monster_template_swimming_level(index: libc::c_long) -> u8 {
    ((get_template(index).cmove & 0x00000700) / 256) as u8
}

// -- Attribute check ----------------------------------------------------------

/// Check if the monster template at `index` has the given attribute.
/// Returns `false` for out-of-bounds indices or unknown attributes.
#[no_mangle]
pub extern "C" fn monster_template_has_attribute_at(
    index: libc::c_long,
    attribute: libc::c_int,
) -> bool {
    let attr = match monster_attribute_from_c(attribute) {
        Some(a) => a,
        None => return false,
    };
    match super::MONSTER_TEMPLATES.get(index as usize) {
        Some(t) => t.has_attribute(attr),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_c_array_macro_null_terminates() {
        let arr: [i8; 28] = str_to_c_array!("Kobold", 28);
        assert_eq!(arr[0], b'K' as i8);
        assert_eq!(arr[5], b'd' as i8);
        assert_eq!(arr[6], 0); // null terminator
    }

    #[test]
    fn test_monster_attribute_from_c_valid() {
        assert_eq!(
            monster_attribute_from_c(0),
            Some(MonsterAttribute::MoveOnlyToAttack)
        );
        assert_eq!(monster_attribute_from_c(29), Some(MonsterAttribute::Evil));
        assert_eq!(
            monster_attribute_from_c(41),
            Some(MonsterAttribute::Regenerates)
        );
    }

    #[test]
    fn test_monster_attribute_from_c_invalid() {
        assert_eq!(monster_attribute_from_c(-1), None);
        assert_eq!(monster_attribute_from_c(42), None);
        assert_eq!(monster_attribute_from_c(100), None);
    }

    /// `monster_template_count()` returns the correct count via C ABI.
    #[test]
    fn monster_template_count_returns_correct_value() {
        assert_eq!(monster_template_count(), 392);
    }

    #[test]
    fn get_name_returns_expected_string() {
        let name_ptr = monster_template_get_name(0);
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(name, "<<Placeholder>>");
    }

    #[test]
    fn get_name_returns_warden_occultist_at_index_1() {
        let name_ptr = monster_template_get_name(1);
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(name, "Town Wizard");
    }

    #[test]
    fn get_symbol_returns_expected_char() {
        assert_eq!(monster_template_get_symbol(0), b'p' as libc::c_char);
    }

    #[test]
    fn get_level_returns_expected_value() {
        assert_eq!(monster_template_get_level(0), 0);
        assert_eq!(monster_template_get_level(391), 100);
    }

    #[test]
    fn get_speed_returns_expected_value() {
        assert_eq!(
            monster_template_get_speed(0),
            super::super::MONSTER_TEMPLATES[0].speed
        );
    }

    #[test]
    fn get_ac_returns_expected_value() {
        assert_eq!(monster_template_get_ac(0), 1);
    }

    #[test]
    fn get_mexp_returns_expected_value() {
        assert_eq!(monster_template_get_mexp(0), 50);
    }

    #[test]
    fn get_sleep_returns_expected_value() {
        assert_eq!(
            monster_template_get_sleep(0),
            super::super::MONSTER_TEMPLATES[0].sleep
        );
    }

    #[test]
    fn get_area_effect_radius_returns_expected_value() {
        assert_eq!(monster_template_get_area_effect_radius(0), 10);
    }

    #[test]
    fn get_magic_resistance_returns_expected_value() {
        assert_eq!(monster_template_get_magic_resistance(0), 20);
    }

    #[test]
    fn get_hit_die_returns_expected_string() {
        let ptr = monster_template_get_hit_die(0);
        let s = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_str().unwrap();
        assert_eq!(s, super::super::MONSTER_TEMPLATES[0].hit_die);
    }

    #[test]
    fn get_damage_returns_expected_string() {
        let ptr = monster_template_get_damage(0);
        let s = unsafe { std::ffi::CStr::from_ptr(ptr) }.to_str().unwrap();
        assert_eq!(s, super::super::MONSTER_TEMPLATES[0].damage);
    }

    #[test]
    fn has_attribute_at_balrog_is_evil() {
        let balrog_template_idx = 391;
        assert!(monster_template_has_attribute_at(
            balrog_template_idx,
            MonsterAttribute::Evil as libc::c_int
        ));
    }

    #[test]
    fn has_attribute_at_balrog_wins_the_game() {
        let balrog_template_idx = 391;
        assert!(monster_template_has_attribute_at(
            balrog_template_idx,
            MonsterAttribute::WinsTheGame as libc::c_int,
        ));
    }

    #[test]
    fn has_attribute_at_balrog_does_not_multiply() {
        let balrog_template_idx = 391;
        assert!(!monster_template_has_attribute_at(
            balrog_template_idx,
            MonsterAttribute::Multiplies as libc::c_int,
        ));
    }

    #[test]
    fn has_attribute_at_oob_returns_false() {
        let out_of_bounds_idx = 9999;
        assert!(!monster_template_has_attribute_at(
            out_of_bounds_idx,
            MonsterAttribute::Evil as libc::c_int,
        ));
    }

    /// Invalid attribute enum value returns false.
    #[test]
    fn has_attribute_at_invalid_attribute_returns_false() {
        let balrog_template_idx = 391;
        let out_of_bounds_idx = 999;
        assert!(!monster_template_has_attribute_at(
            balrog_template_idx,
            out_of_bounds_idx
        ));
    }

    #[test]
    fn out_of_bounds_returns_glitch_name() {
        let out_of_bounds_idx = 9999;
        let name_ptr = monster_template_get_name(out_of_bounds_idx);
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(name, "Glitch");
    }

    #[test]
    fn out_of_bounds_returns_glitch_symbol() {
        let out_of_bounds_idx = 9999;
        assert_eq!(
            monster_template_get_symbol(out_of_bounds_idx),
            b'?' as libc::c_char
        );
    }

    #[test]
    fn out_of_bounds_returns_glitch_level() {
        let out_of_bounds_idx = 9999;
        assert_eq!(monster_template_get_level(out_of_bounds_idx), 0);
    }

    /// Town Wizard (index 1) has spells (0x00009F52), so raw value is non-zero.
    #[test]
    fn get_spells_raw_returns_expected_value() {
        let town_wizard_template_idx = 1;
        assert_eq!(
            monster_template_get_spells_raw(town_wizard_template_idx),
            0x00009F52
        );
    }

    /// Index 2 has spells == 0, so raw value is zero.
    #[test]
    fn get_spells_raw_returns_zero_for_no_spells() {
        let no_spells_template_idx = 2;
        assert_eq!(monster_template_get_spells_raw(no_spells_template_idx), 0);
    }

    /// OOB returns zero for spells.
    #[test]
    fn get_spells_raw_oob_returns_zero() {
        let out_of_bounds_idx = 9999;
        assert_eq!(monster_template_get_spells_raw(out_of_bounds_idx), 0);
    }

    /// Town Wizard (index 1, spells=0x00009F52) has spells.
    #[test]
    fn has_spells_true_for_town_wizard() {
        let town_wizard_template_idx = 1;
        assert!(monster_template_has_spells(town_wizard_template_idx));
    }

    /// Index 2 (spells=0) has no spells.
    #[test]
    fn has_spells_false_when_no_spells() {
        let no_spells_template_idx = 2;
        assert!(!monster_template_has_spells(no_spells_template_idx));
    }

    /// Town Wizard (index 1, spells=0x00009F52): frequency = 0x2.
    #[test]
    fn spell_frequency_returns_expected_value() {
        let town_wizard_template_idx = 1;
        assert_eq!(
            monster_template_spell_frequency(town_wizard_template_idx),
            2
        );
    }

    /// Balrog (index 391, spells=0x0281C743): frequency = 0x3.
    #[test]
    fn spell_frequency_balrog() {
        let balrog_template_idx = 391;
        assert_eq!(monster_template_spell_frequency(balrog_template_idx), 3);
    }

    /// Town Wizard (index 1, spells=0x00009F52): bit 31 is not set.
    #[test]
    fn spell_frequency_not_inverted_for_town_wizard() {
        let town_wizard_template_idx = 1;
        assert!(!monster_template_spell_frequency_is_inverted(
            town_wizard_template_idx
        ));
    }

    /// Find a monster whose spells field has bit 31 set (inverted frequency).
    #[test]
    fn spell_frequency_inverted_when_bit31_set() {
        let inverted_idx = super::super::MONSTER_TEMPLATES
            .iter()
            .position(|t| t.spells & 0x80000000 != 0)
            .expect("at least one template should have inverted spell frequency");
        assert!(monster_template_spell_frequency_is_inverted(
            inverted_idx as libc::c_long
        ));
    }

    /// Town Wizard (index 1, spells=0x00009F52): choice bits = 0x9F50.
    #[test]
    fn spell_choice_bits_returns_expected_value() {
        let town_wizard_template_idx = 1;
        assert_eq!(
            monster_template_spell_choice_bits(town_wizard_template_idx),
            0x00009F50
        );
    }

    /// Balrog (index 391, spells=0x0281C743): choice bits = 0x0281C740.
    #[test]
    fn spell_choice_bits_balrog() {
        let balrog_template_idx = 391;
        assert_eq!(
            monster_template_spell_choice_bits(balrog_template_idx),
            0x0281C740
        );
    }

    /// Balrog (index 391) has cmove = 0xFF1F0300.
    #[test]
    fn get_cmove_returns_expected_value() {
        let balrog_template_idx = 391;
        assert_eq!(monster_template_get_cmove(balrog_template_idx), 0xFF1F0300);
    }

    /// OOB returns zero for cmove.
    #[test]
    fn get_cmove_oob_returns_zero() {
        let out_of_bounds_idx = 9999;
        assert_eq!(monster_template_get_cmove(out_of_bounds_idx), 0);
    }

    /// Placeholder (index 0) has cdefense = 0x3000.
    #[test]
    fn get_cdefense_returns_expected_value() {
        let placeholder_template_idx = 0;
        assert_eq!(
            monster_template_get_cdefense(placeholder_template_idx),
            0x3000
        );
    }

    /// OOB returns zero for cdefense.
    #[test]
    fn get_cdefense_oob_returns_zero() {
        let out_of_bounds_idx = 9999;
        assert_eq!(monster_template_get_cdefense(out_of_bounds_idx), 0);
    }

    /// Balrog (index 391, cmove=0xFF1F0300): movement_speed = (0x300 & 0x300) / 256 = 3.
    #[test]
    fn movement_speed_returns_expected_value() {
        let balrog_template_idx = 391;
        assert_eq!(monster_template_movement_speed(balrog_template_idx), 3);
    }

    /// Placeholder (index 0, cmove=0x0010C000): movement_speed = 0.
    #[test]
    fn movement_speed_zero_for_placeholder() {
        let placeholder_template_idx = 0;
        assert_eq!(monster_template_movement_speed(placeholder_template_idx), 0);
    }

    /// Balrog (index 391, cmove=0xFF1F0300): swimming_level = (0x300 & 0x700) / 256 = 3.
    #[test]
    fn swimming_level_returns_expected_value() {
        let balrog_template_idx = 391;
        assert_eq!(monster_template_swimming_level(balrog_template_idx), 3);
    }

    /// Placeholder (index 0, cmove=0x0010C000): swimming_level = 0.
    #[test]
    fn swimming_level_zero_for_placeholder() {
        let placeholder_template_idx = 0;
        assert_eq!(monster_template_swimming_level(placeholder_template_idx), 0);
    }
}
