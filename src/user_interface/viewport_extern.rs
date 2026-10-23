use crate::user_interface::viewport::{
    calculate_viewport, contains_screen_cell, current_map_rect, update_viewport,
};

#[repr(C)]
pub struct CViewportState {
    pub panel_row: libc::c_long,
    pub panel_col: libc::c_long,
    pub world_top: libc::c_long,
    pub world_bottom: libc::c_long,
    pub world_left: libc::c_long,
    pub world_right: libc::c_long,
    pub screen_row: libc::c_long,
    pub screen_col: libc::c_long,
    pub map_row: libc::c_long,
    pub map_col: libc::c_long,
    pub map_height: libc::c_long,
    pub map_width: libc::c_long,
    pub panel_row_prt: libc::c_long,
    pub panel_col_prt: libc::c_long,
    pub max_panel_rows: libc::c_long,
    pub max_panel_cols: libc::c_long,
    pub changed: bool,
}

impl CViewportState {
    fn from_viewport(viewport: crate::user_interface::viewport::Viewport, changed: bool) -> Self {
        let map = current_map_rect();
        Self {
            panel_row: viewport.panel_row as libc::c_long,
            panel_col: viewport.panel_col as libc::c_long,
            world_top: viewport.world_top as libc::c_long,
            world_bottom: viewport.world_bottom as libc::c_long,
            world_left: viewport.world_left as libc::c_long,
            world_right: viewport.world_right as libc::c_long,
            screen_row: viewport.screen_row as libc::c_long,
            screen_col: viewport.screen_col as libc::c_long,
            map_row: map.row as libc::c_long,
            map_col: map.col as libc::c_long,
            map_height: map.height as libc::c_long,
            map_width: map.width as libc::c_long,
            panel_row_prt: viewport.panel_row_prt as libc::c_long,
            panel_col_prt: viewport.panel_col_prt as libc::c_long,
            max_panel_rows: viewport.max_panel_rows as libc::c_long,
            max_panel_cols: viewport.max_panel_cols as libc::c_long,
            changed,
        }
    }
}

#[no_mangle]
pub extern "C" fn C_viewport_get(
    panel_row: libc::c_long,
    panel_col: libc::c_long,
    world_height: libc::c_long,
    world_width: libc::c_long,
) -> CViewportState {
    CViewportState::from_viewport(
        calculate_viewport(
            current_map_rect(),
            panel_row as i32,
            panel_col as i32,
            world_height as i32,
            world_width as i32,
        ),
        false,
    )
}

#[no_mangle]
pub extern "C" fn C_viewport_update(
    panel_row: libc::c_long,
    panel_col: libc::c_long,
    world_height: libc::c_long,
    world_width: libc::c_long,
    y: libc::c_long,
    x: libc::c_long,
    force: bool,
) -> CViewportState {
    let (viewport, changed) = update_viewport(
        current_map_rect(),
        panel_row as i32,
        panel_col as i32,
        world_height as i32,
        world_width as i32,
        y as i32,
        x as i32,
        force,
    );
    CViewportState::from_viewport(viewport, changed)
}

#[no_mangle]
pub extern "C" fn C_viewport_screen_contains(row: libc::c_long, col: libc::c_long) -> bool {
    contains_screen_cell(current_map_rect(), row as i32, col as i32)
}
