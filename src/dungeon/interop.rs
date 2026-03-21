//! C ABI wrappers for dungeon light helpers.

use crate::dungeon::light::should_light_cell;

extern "C" {
    fn dungeon_light_is_open(row: libc::c_long, col: libc::c_long) -> libc::c_int;
}

#[no_mangle]
pub extern "C" fn dungeon_light_should_light_cell(
    player_row: libc::c_long,
    player_col: libc::c_long,
    target_row: libc::c_long,
    target_col: libc::c_long,
    light_radius: libc::c_long,
) -> libc::c_int {
    let visible = should_light_cell(
        player_row,
        player_col,
        target_row,
        target_col,
        light_radius,
        |row, col| unsafe { dungeon_light_is_open(row as libc::c_long, col as libc::c_long) != 0 },
    );

    if visible {
        1
    } else {
        0
    }
}
