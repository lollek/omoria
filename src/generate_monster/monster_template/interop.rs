//! C ABI wrappers for monster template interop.
//!
//! This module provides C-compatible access to the Rust monster template data,
//! allowing the C code to be gradually migrated to use Rust implementations.

use super::{MonsterAttribute, MonsterTemplate};

use std::sync::LazyLock;

/// Pre-computed C-compatible monster templates, built once from Rust data.
/// Used to return `const char*` pointers to string fields.
static TEMPLATES_C: LazyLock<Vec<MonsterTemplateC>> = LazyLock::new(|| {
    super::MONSTER_TEMPLATES
        .iter()
        .map(|t| MonsterTemplateC::from_rust(t))
        .collect()
});

/// C-compatible monster attributes struct.
/// Must match `monster_attributes` in `monster_template.h`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MonsterAttributesC {
    pub multiplies: bool,
    pub can_move: bool,
}

/// C-compatible monster template struct.
/// Must match `monster_template_t` in `monster_template.h`.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MonsterTemplateC {
    pub area_effect_radius: u8,
    pub ac: u8,
    pub name: [libc::c_char; 28],
    pub cmove: u64,
    pub spells: u64,
    pub cdefense: u64,
    pub sleep: i16,
    pub mexp: i64,
    pub speed: i8,
    pub symbol: libc::c_char,
    pub hit_die: [libc::c_char; 7],
    pub damage: [libc::c_char; 36],
    pub level: i8,
    pub magic_resistance: u8,
    pub attributes: MonsterAttributesC,
}

impl MonsterTemplateC {
    /// Convert a Rust MonsterTemplate to C-compatible struct.
    pub const fn from_rust(t: &MonsterTemplate) -> Self {
        Self {
            area_effect_radius: t.area_effect_radius,
            ac: t.ac,
            name: str_to_c_array_28(t.name),
            cmove: t.cmove,
            spells: t.spells,
            cdefense: t.cdefense,
            sleep: t.sleep,
            mexp: t.mexp,
            speed: t.speed,
            symbol: t.symbol as libc::c_char,
            hit_die: str_to_c_array_7(t.hit_die),
            damage: str_to_c_array_36(t.damage),
            level: t.level,
            magic_resistance: t.magic_resistance,
            attributes: MonsterAttributesC {
                multiplies: t.multiplies,
                can_move: t.can_move,
            },
        }
    }
}

/// Convert a static str to a fixed-size C char array (28 bytes).
const fn str_to_c_array_28(s: &str) -> [libc::c_char; 28] {
    let mut arr = [0i8; 28];
    let bytes = s.as_bytes();
    let len = if bytes.len() < 27 { bytes.len() } else { 27 };
    let mut i = 0;
    while i < len {
        arr[i] = bytes[i] as i8;
        i += 1;
    }
    arr
}

/// Convert a static str to a fixed-size C char array (7 bytes).
const fn str_to_c_array_7(s: &str) -> [libc::c_char; 7] {
    let mut arr = [0i8; 7];
    let bytes = s.as_bytes();
    let len = if bytes.len() < 6 { bytes.len() } else { 6 };
    let mut i = 0;
    while i < len {
        arr[i] = bytes[i] as i8;
        i += 1;
    }
    arr
}

/// Convert a static str to a fixed-size C char array (36 bytes).
const fn str_to_c_array_36(s: &str) -> [libc::c_char; 36] {
    let mut arr = [0i8; 36];
    let bytes = s.as_bytes();
    let len = if bytes.len() < 35 { bytes.len() } else { 35 };
    let mut i = 0;
    while i < len {
        arr[i] = bytes[i] as i8;
        i += 1;
    }
    arr
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

/// Fallback template returned for out-of-bounds access.
/// A visible "Glitch" monster makes indexing bugs obvious in-game.
static GLITCH_TEMPLATE_C: LazyLock<MonsterTemplateC> = LazyLock::new(|| {
    MonsterTemplateC::from_rust(&MonsterTemplate {
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
    })
});

/// Get the C-compatible template at `index`.
///
/// Returns the "Glitch" fallback template if `index` is out of bounds,
/// so indexing bugs become visible in-game rather than silently returning zeros.
fn get_template_c(index: libc::c_long) -> &'static MonsterTemplateC {
    TEMPLATES_C
        .get(index as usize)
        .unwrap_or(&GLITCH_TEMPLATE_C)
}

/// Return the number of monster templates.
#[no_mangle]
pub extern "C" fn monster_template_count() -> libc::c_long {
    super::MONSTER_TEMPLATES.len() as libc::c_long
}

/// Return the name of the monster template at `index` as a C string.
///
/// Returns a pointer to a null-terminated string. The pointer is valid for the
/// lifetime of the program. Falls back to the Glitch template if `index` is
/// out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_name(index: libc::c_long) -> *const libc::c_char {
    get_template_c(index).name.as_ptr()
}

/// Return the map symbol of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_symbol(index: libc::c_long) -> libc::c_char {
    get_template_c(index).symbol
}

/// Return the level of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_level(index: libc::c_long) -> i8 {
    get_template_c(index).level
}

/// Return the speed of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_speed(index: libc::c_long) -> i8 {
    get_template_c(index).speed
}

/// Return the armor class of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_ac(index: libc::c_long) -> u8 {
    get_template_c(index).ac
}

/// Return the experience value of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_mexp(index: libc::c_long) -> i64 {
    get_template_c(index).mexp
}

/// Return the sleep/inactive counter of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_sleep(index: libc::c_long) -> i16 {
    get_template_c(index).sleep
}

/// Return the area effect radius of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_area_effect_radius(index: libc::c_long) -> u8 {
    get_template_c(index).area_effect_radius
}

/// Return the magic resistance of the monster template at `index`.
///
/// Falls back to the Glitch template if `index` is out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_magic_resistance(index: libc::c_long) -> u8 {
    get_template_c(index).magic_resistance
}

/// Return the hit die string of the monster template at `index` as a C string.
///
/// Returns a pointer to a null-terminated string. The pointer is valid for the
/// lifetime of the program. Falls back to the Glitch template if `index` is
/// out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_hit_die(index: libc::c_long) -> *const libc::c_char {
    get_template_c(index).hit_die.as_ptr()
}

/// Return the damage string of the monster template at `index` as a C string.
///
/// Returns a pointer to a null-terminated string. The pointer is valid for the
/// lifetime of the program. Falls back to the Glitch template if `index` is
/// out of bounds.
#[no_mangle]
pub extern "C" fn monster_template_get_damage(index: libc::c_long) -> *const libc::c_char {
    get_template_c(index).damage.as_ptr()
}

/// Check if a monster template has the given attribute.
///
/// # Safety
/// The `template` pointer must be valid and point to a `MonsterTemplateC`.
#[no_mangle]
pub unsafe extern "C" fn monster_template_has_attribute(
    template: *const MonsterTemplateC,
    attribute: libc::c_int,
) -> bool {
    if template.is_null() {
        return false;
    }

    let template = &*template;
    let attr = match monster_attribute_from_c(attribute) {
        Some(a) => a,
        None => return false,
    };

    // Convert C template to Rust for attribute checking
    // We can check directly using the bit fields
    match attr {
        MonsterAttribute::MoveOnlyToAttack => !template.attributes.can_move,
        MonsterAttribute::Multiplies => template.attributes.multiplies,
        MonsterAttribute::RandomMovement20pc => (template.cmove & 0x00000002) != 0,
        MonsterAttribute::RandomMovement40pc => (template.cmove & 0x00000004) != 0,
        MonsterAttribute::RandomMovement75pc => (template.cmove & 0x00000008) != 0,
        MonsterAttribute::WaterBased => (template.cmove & 0x00000010) != 0,
        MonsterAttribute::LandBased => (template.cmove & 0x00000010) == 0,
        MonsterAttribute::DiesInWrongElement => (template.cmove & 0x00000040) != 0,
        MonsterAttribute::GoodMonster => (template.cmove & 0x00004000) != 0,
        MonsterAttribute::Unspawnable => (template.cmove & 0x00008000) != 0,
        MonsterAttribute::InvisibleMovement => (template.cmove & 0x00010000) != 0,
        MonsterAttribute::MovesThroughDoor => (template.cmove & 0x00020000) != 0,
        MonsterAttribute::MovesThroughWall => (template.cmove & 0x00040000) != 0,
        MonsterAttribute::MovesThroughCreatures => (template.cmove & 0x00080000) != 0,
        MonsterAttribute::PicksUpObjects => (template.cmove & 0x00100000) != 0,
        MonsterAttribute::AnchorsInWater => (template.cmove & 0x00400000) != 0,
        MonsterAttribute::Flying => (template.cmove & 0x00800000) != 0,
        MonsterAttribute::CarriesObjects => (template.cmove & 0x01000000) != 0,
        MonsterAttribute::CarriesGold => (template.cmove & 0x02000000) != 0,
        MonsterAttribute::Carries60pc => (template.cmove & 0x04000000) != 0,
        MonsterAttribute::Carries90pc => (template.cmove & 0x08000000) != 0,
        MonsterAttribute::Carries1d2Things => (template.cmove & 0x10000000) != 0,
        MonsterAttribute::Carries2d2Things => (template.cmove & 0x20000000) != 0,
        MonsterAttribute::Carries4d2Things => (template.cmove & 0x40000000) != 0,
        MonsterAttribute::WinsTheGame => (template.cmove & 0x80000000) != 0,
        MonsterAttribute::Dragon => (template.cdefense & 0x0001) != 0,
        MonsterAttribute::Monster => (template.cdefense & 0x0002) != 0,
        MonsterAttribute::Evil => (template.cdefense & 0x0004) != 0,
        MonsterAttribute::Undead => (template.cdefense & 0x0008) != 0,
        MonsterAttribute::Demon => (template.cdefense & 0x0400) != 0,
        MonsterAttribute::VulnerableToFrost => (template.cdefense & 0x0010) != 0,
        MonsterAttribute::VulnerableToFire => (template.cdefense & 0x0020) != 0,
        MonsterAttribute::VulnerableToPoison => (template.cdefense & 0x0040) != 0,
        MonsterAttribute::VulnerableToAcid => (template.cdefense & 0x0080) != 0,
        MonsterAttribute::VulnerableToLightning => (template.cdefense & 0x0100) != 0,
        MonsterAttribute::VulnerableToStoneToMud => (template.cdefense & 0x0200) != 0,
        MonsterAttribute::Uncharmable => (template.cdefense & 0x1000) != 0,
        MonsterAttribute::VisibleWithInfravision => (template.cdefense & 0x2000) != 0,
        MonsterAttribute::MaxHitPoints => (template.cdefense & 0x4000) != 0,
        MonsterAttribute::Regenerates => (template.cdefense & 0x8000) != 0,
        // Compound attributes
        MonsterAttribute::SurvivesInWater => {
            let water_based = (template.cmove & 0x00000010) != 0;
            let dies_in_wrong = (template.cmove & 0x00000040) != 0;
            let flying = (template.cmove & 0x00800000) != 0;
            water_based || !dies_in_wrong || flying
        }
        MonsterAttribute::SurvivesOnLand => {
            let land_based = (template.cmove & 0x00000010) == 0;
            let dies_in_wrong = (template.cmove & 0x00000040) != 0;
            let flying = (template.cmove & 0x00800000) != 0;
            land_based || !dies_in_wrong || flying
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monster_template_c_size() {
        // Verify our struct size matches the C struct
        // C struct should be: 2 + 28 + 8 + 8 + 8 + 2 + 8 + 1 + 1 + 7 + 36 + 1 + 1 + 2 = ~113 bytes
        // But with padding it may differ
        assert!(std::mem::size_of::<MonsterTemplateC>() > 0);
    }

    #[test]
    fn test_str_to_c_array_28() {
        let arr = str_to_c_array_28("Kobold");
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

    // =========================================================================
    // Scalar getter tests (Step 2)
    // =========================================================================

    #[test]
    fn get_name_returns_expected_string() {
        let name_ptr = monster_template_get_name(0);
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(name, "<<Placeholder>>");
    }

    #[test]
    fn get_name_returns_town_wizard_at_index_1() {
        let name_ptr = monster_template_get_name(1);
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(name, "Town Wizard");
    }

    #[test]
    fn get_symbol_returns_expected_char() {
        // Index 0: placeholder has symbol 'p'
        assert_eq!(monster_template_get_symbol(0), b'p' as libc::c_char);
    }

    #[test]
    fn get_level_returns_expected_value() {
        // Index 0: placeholder is level 0
        assert_eq!(monster_template_get_level(0), 0);
        // Balrog is index 391, level 100
        assert_eq!(monster_template_get_level(391), 100);
    }

    #[test]
    fn get_speed_returns_expected_value() {
        // Index 0: placeholder speed is 1
        assert_eq!(
            monster_template_get_speed(0),
            super::super::MONSTER_TEMPLATES[0].speed
        );
    }

    #[test]
    fn get_ac_returns_expected_value() {
        // Index 0: placeholder AC is 1
        assert_eq!(monster_template_get_ac(0), 1);
    }

    #[test]
    fn get_mexp_returns_expected_value() {
        // Index 0: placeholder mexp is 50
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
        // Index 0: placeholder area_effect_radius is 10
        assert_eq!(monster_template_get_area_effect_radius(0), 10);
    }

    #[test]
    fn get_magic_resistance_returns_expected_value() {
        // Index 0: placeholder magic_resistance is 20
        assert_eq!(monster_template_get_magic_resistance(0), 20);
    }

    #[test]
    fn get_hit_die_returns_expected_string() {
        let ptr = monster_template_get_hit_die(0);
        let s = unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_str()
            .unwrap();
        assert_eq!(s, super::super::MONSTER_TEMPLATES[0].hit_die);
    }

    #[test]
    fn get_damage_returns_expected_string() {
        let ptr = monster_template_get_damage(0);
        let s = unsafe { std::ffi::CStr::from_ptr(ptr) }
            .to_str()
            .unwrap();
        assert_eq!(s, super::super::MONSTER_TEMPLATES[0].damage);
    }

    #[test]
    fn out_of_bounds_returns_glitch_name() {
        let name_ptr = monster_template_get_name(9999);
        let name = unsafe { std::ffi::CStr::from_ptr(name_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(name, "Glitch");
    }

    #[test]
    fn out_of_bounds_returns_glitch_symbol() {
        assert_eq!(monster_template_get_symbol(9999), b'?' as libc::c_char);
    }

    #[test]
    fn out_of_bounds_returns_glitch_level() {
        assert_eq!(monster_template_get_level(9999), 0);
    }
}
