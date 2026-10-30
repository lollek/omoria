use super::map_colors::{map_color_attr, terrain_color, MapColor};
use crate::model::FloorType;
use std::convert::TryFrom;

#[no_mangle]
pub extern "C" fn C_map_color_water_attr() -> pancurses::chtype {
    map_color_attr(MapColor::Water)
}

#[no_mangle]
pub extern "C" fn C_map_color_init(color_count: libc::c_int) {
    super::map_colors::init_map_colors(color_count);
}

pub(crate) fn terrain_attr_for_type(floor_type: FloorType) -> pancurses::chtype {
    terrain_color(floor_type)
        .map(map_color_attr)
        .unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn C_map_color_terrain_attr(fval: u8) -> pancurses::chtype {
    FloorType::try_from(fval)
        .ok()
        .map(terrain_attr_for_type)
        .unwrap_or_default()
}
