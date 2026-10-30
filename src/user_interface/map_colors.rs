use pancurses::chtype;
use std::sync::Mutex;

pub const FIRST_MAP_PAIR: i16 = 16;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MapColor {
    Water,
    Granite,
    Magma,
    Quartz,
    BoundaryWall,
    RoomFloor,
    Corridor,
    Door,
    Stairs,
    Rubble,
    Trap,
    ShopEntrance,
    Gold,
    Gem,
    Potion,
    Scroll,
    Weapon,
    Armor,
    Food,
    Item,
    MonsterWhite,
    MonsterRed,
    MonsterGreen,
    MonsterYellow,
    MonsterBlue,
    MonsterMagenta,
    MonsterCyan,
    MonsterOrange,
    MonsterBrown,
    MonsterGray,
    MonsterPurple,
    MonsterPink,
}

impl MapColor {
    pub const ALL: [MapColor; 32] = [
        MapColor::Water,
        MapColor::Granite,
        MapColor::Magma,
        MapColor::Quartz,
        MapColor::BoundaryWall,
        MapColor::RoomFloor,
        MapColor::Corridor,
        MapColor::Door,
        MapColor::Stairs,
        MapColor::Rubble,
        MapColor::Trap,
        MapColor::ShopEntrance,
        MapColor::Gold,
        MapColor::Gem,
        MapColor::Potion,
        MapColor::Scroll,
        MapColor::Weapon,
        MapColor::Armor,
        MapColor::Food,
        MapColor::Item,
        MapColor::MonsterWhite,
        MapColor::MonsterRed,
        MapColor::MonsterGreen,
        MapColor::MonsterYellow,
        MapColor::MonsterBlue,
        MapColor::MonsterMagenta,
        MapColor::MonsterCyan,
        MapColor::MonsterOrange,
        MapColor::MonsterBrown,
        MapColor::MonsterGray,
        MapColor::MonsterPurple,
        MapColor::MonsterPink,
    ];
}

/// One role's colors: a 256-color index and the 8-color fallback with optional bold.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColorSpec {
    pub index256: i16,
    pub basic: i16,
    pub bold: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Palette {
    Basic,
    Extended,
}

const fn spec(index256: i16, basic: i16, bold: bool) -> ColorSpec {
    ColorSpec {
        index256,
        basic,
        bold,
    }
}

pub fn color_spec(role: MapColor) -> ColorSpec {
    use pancurses::{COLOR_BLUE, COLOR_CYAN, COLOR_GREEN, COLOR_MAGENTA, COLOR_RED, COLOR_YELLOW};
    const COLOR_WHITE: i16 = pancurses::COLOR_WHITE;
    match role {
        MapColor::Water => spec(4, COLOR_BLUE, false),
        MapColor::Granite => spec(245, COLOR_WHITE, false),
        MapColor::Magma => spec(130, COLOR_RED, false),
        MapColor::Quartz => spec(195, COLOR_CYAN, false),
        MapColor::BoundaryWall => spec(94, COLOR_YELLOW, false),
        MapColor::RoomFloor => spec(240, COLOR_WHITE, false),
        MapColor::Corridor => spec(108, COLOR_GREEN, false),
        MapColor::Door => spec(172, COLOR_YELLOW, true),
        MapColor::Stairs => spec(231, COLOR_WHITE, true),
        MapColor::Rubble => spec(143, COLOR_YELLOW, false),
        MapColor::Trap => spec(196, COLOR_RED, true),
        MapColor::ShopEntrance => spec(213, COLOR_MAGENTA, true),
        MapColor::Gold => spec(220, COLOR_YELLOW, true),
        MapColor::Gem => spec(51, COLOR_CYAN, true),
        MapColor::Potion => spec(129, COLOR_MAGENTA, false),
        MapColor::Scroll => spec(187, COLOR_WHITE, false),
        MapColor::Weapon => spec(117, COLOR_CYAN, false),
        MapColor::Armor => spec(69, COLOR_BLUE, true),
        MapColor::Food => spec(70, COLOR_GREEN, false),
        MapColor::Item => spec(247, COLOR_WHITE, false),
        MapColor::MonsterWhite => spec(255, COLOR_WHITE, true),
        MapColor::MonsterRed => spec(160, COLOR_RED, false),
        MapColor::MonsterGreen => spec(34, COLOR_GREEN, false),
        MapColor::MonsterYellow => spec(226, COLOR_YELLOW, true),
        MapColor::MonsterBlue => spec(27, COLOR_BLUE, true),
        MapColor::MonsterMagenta => spec(164, COLOR_MAGENTA, false),
        MapColor::MonsterCyan => spec(44, COLOR_CYAN, false),
        MapColor::MonsterOrange => spec(208, COLOR_RED, true),
        MapColor::MonsterBrown => spec(130, COLOR_YELLOW, false),
        MapColor::MonsterGray => spec(246, COLOR_WHITE, false),
        MapColor::MonsterPurple => spec(93, COLOR_MAGENTA, true),
        MapColor::MonsterPink => spec(218, COLOR_MAGENTA, true),
    }
}

/// Fewer than 8 colors (or none) leaves the map uncolored.
pub fn palette_for(color_count: i32) -> Option<Palette> {
    match color_count {
        256.. => Some(Palette::Extended),
        8..=255 => Some(Palette::Basic),
        _ => None,
    }
}

pub fn pair_number(role: MapColor) -> i16 {
    FIRST_MAP_PAIR + role as i16
}

pub fn foreground(role: MapColor, palette: Palette) -> i16 {
    let spec = color_spec(role);
    match palette {
        Palette::Basic => spec.basic,
        Palette::Extended => spec.index256,
    }
}

pub fn attr_for(role: MapColor, palette: Palette) -> chtype {
    let bold = palette == Palette::Basic && color_spec(role).bold;
    let attr = pancurses::COLOR_PAIR(pair_number(role) as chtype);
    if bold {
        attr | pancurses::A_BOLD
    } else {
        attr
    }
}

static MAP_ATTRS: Mutex<[chtype; MapColor::ALL.len()]> = Mutex::new([0; MapColor::ALL.len()]);

/// Registers one pair per role; leaves every attribute empty if the terminal cannot show them.
pub fn init_map_colors(color_count: i32) {
    let mut attrs = MAP_ATTRS.lock().unwrap();
    *attrs = [0; MapColor::ALL.len()];
    let Some(palette) = palette_for(color_count) else {
        return;
    };
    let registered = MapColor::ALL.iter().all(|&role| {
        pancurses::init_pair(
            pair_number(role),
            foreground(role, palette),
            pancurses::COLOR_BLACK,
        ) == 0
    });
    if registered {
        for role in MapColor::ALL {
            attrs[role as usize] = attr_for(role, palette);
        }
    }
}

pub fn map_color_attr(role: MapColor) -> chtype {
    MAP_ATTRS.lock().unwrap()[role as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_lists_every_role_in_declaration_order() {
        for (index, role) in MapColor::ALL.iter().enumerate() {
            assert_eq!(*role as usize, index, "{role:?} is out of order in ALL");
        }
        assert_eq!(
            MapColor::MonsterPink as usize + 1,
            MapColor::ALL.len(),
            "ALL is missing the last role"
        );
    }

    #[test]
    fn every_role_has_a_visible_256_color_and_a_visible_fallback() {
        for role in MapColor::ALL {
            let spec = color_spec(role);
            assert!(
                spec.index256 > 0 && spec.index256 != 16 && spec.index256 <= 255,
                "{role:?} 256-color index {} is black or out of range",
                spec.index256
            );
            assert!(
                (1..=7).contains(&spec.basic),
                "{role:?} fallback {} is black on black or out of range",
                spec.basic
            );
        }
    }

    #[test]
    fn color_counts_pick_the_palette() {
        assert_eq!(palette_for(0), None);
        assert_eq!(palette_for(-1), None);
        assert_eq!(palette_for(7), None);
        assert_eq!(palette_for(8), Some(Palette::Basic));
        assert_eq!(palette_for(16), Some(Palette::Basic));
        assert_eq!(palette_for(255), Some(Palette::Basic));
        assert_eq!(palette_for(256), Some(Palette::Extended));
        assert_eq!(palette_for(32767), Some(Palette::Extended));
    }

    #[test]
    fn pairs_start_at_16_are_unique_and_fit_an_8_color_terminal() {
        assert_eq!(pair_number(MapColor::ALL[0]), 16);
        for (index, role) in MapColor::ALL.iter().enumerate() {
            assert_eq!(pair_number(*role), 16 + index as i16);
            assert!(
                pair_number(*role) < 64,
                "{role:?} exceeds 64 color pairs: {}",
                pair_number(*role)
            );
        }
    }

    #[test]
    fn palette_picks_the_matching_foreground() {
        assert_eq!(
            foreground(MapColor::Water, Palette::Basic),
            pancurses::COLOR_BLUE
        );
        assert_eq!(foreground(MapColor::Water, Palette::Extended), 4);
    }

    #[test]
    fn water_keeps_its_plain_blue_look_on_an_8_color_terminal() {
        assert_eq!(
            attr_for(MapColor::Water, Palette::Basic),
            pancurses::COLOR_PAIR(pair_number(MapColor::Water) as chtype)
        );
    }

    #[test]
    fn bold_applies_only_to_the_8_color_fallback() {
        let basic = attr_for(MapColor::Stairs, Palette::Basic);
        let extended = attr_for(MapColor::Stairs, Palette::Extended);
        assert_eq!(basic & pancurses::A_BOLD, pancurses::A_BOLD);
        assert_eq!(extended & pancurses::A_BOLD, 0);
    }

    #[test]
    #[serial_test::serial]
    fn terminals_without_colors_get_empty_attributes() {
        init_map_colors(0);
        for role in MapColor::ALL {
            assert_eq!(map_color_attr(role), 0, "{role:?} should be empty");
        }
    }
}
