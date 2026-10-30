use super::map_colors::{map_color_attr, MapColor};

#[no_mangle]
pub extern "C" fn C_map_color_water_attr() -> pancurses::chtype {
    map_color_attr(MapColor::Water)
}
