use super::layout::{current_layout, Rect};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Viewport {
    pub panel_row: i32,
    pub panel_col: i32,
    pub world_top: i32,
    pub world_bottom: i32,
    pub world_left: i32,
    pub world_right: i32,
    pub screen_row: i32,
    pub screen_col: i32,
    pub panel_row_prt: i32,
    pub panel_col_prt: i32,
    pub max_panel_rows: i32,
    pub max_panel_cols: i32,
}

fn max_panel_index(world_extent: i32, viewport_extent: i32) -> i32 {
    if world_extent <= viewport_extent || viewport_extent <= 0 {
        return 0;
    }
    let step = (viewport_extent / 2).max(1);
    (world_extent - viewport_extent + step - 1) / step
}

fn panel_bounds(panel: i32, world_extent: i32, viewport_extent: i32) -> (i32, i32) {
    if world_extent <= 0 || viewport_extent <= 0 {
        return (1, 0);
    }
    let visible_extent = world_extent.min(viewport_extent);
    let max_top = world_extent - visible_extent + 1;
    let top = (panel.max(0) * (viewport_extent / 2).max(1) + 1).min(max_top);
    (top, top + visible_extent - 1)
}

pub fn calculate_viewport(
    map: Rect,
    panel_row: i32,
    panel_col: i32,
    world_height: i32,
    world_width: i32,
) -> Viewport {
    let max_panel_rows = max_panel_index(world_height, map.height);
    let max_panel_cols = max_panel_index(world_width, map.width);
    let panel_row = panel_row.clamp(0, max_panel_rows);
    let panel_col = panel_col.clamp(0, max_panel_cols);
    let (world_top, world_bottom) = panel_bounds(panel_row, world_height, map.height);
    let (world_left, world_right) = panel_bounds(panel_col, world_width, map.width);
    let screen_row = map.row + (map.height - (world_bottom - world_top + 1)).max(0) / 2;
    let screen_col = map.col + (map.width - (world_right - world_left + 1)).max(0) / 2;

    Viewport {
        panel_row,
        panel_col,
        world_top,
        world_bottom,
        world_left,
        world_right,
        screen_row,
        screen_col,
        panel_row_prt: world_top - screen_row - 1,
        panel_col_prt: world_left - screen_col - 1,
        max_panel_rows,
        max_panel_cols,
    }
}

fn next_panel_index(
    current: i32,
    position: i32,
    bounds: (i32, i32),
    viewport_extent: i32,
    max_panel: i32,
    force: bool,
    leading_margin: i32,
) -> i32 {
    if !force && position >= bounds.0 + leading_margin && position <= bounds.1 - leading_margin {
        return current;
    }
    ((position - leading_margin).max(0) / (viewport_extent / 2).max(1)).clamp(0, max_panel)
}

pub fn update_viewport(
    map: Rect,
    current_panel_row: i32,
    current_panel_col: i32,
    world_height: i32,
    world_width: i32,
    y: i32,
    x: i32,
    force: bool,
) -> (Viewport, bool) {
    let current = calculate_viewport(
        map,
        current_panel_row,
        current_panel_col,
        world_height,
        world_width,
    );
    let panel_row = next_panel_index(
        current.panel_row,
        y,
        (current.world_top, current.world_bottom),
        map.height,
        current.max_panel_rows,
        force,
        2,
    );
    let panel_col = next_panel_index(
        current.panel_col,
        x,
        (current.world_left, current.world_right),
        map.width,
        current.max_panel_cols,
        force,
        3,
    );
    let changed = panel_row != current.panel_row || panel_col != current.panel_col;
    (
        calculate_viewport(map, panel_row, panel_col, world_height, world_width),
        changed,
    )
}

pub fn current_map_rect() -> Rect {
    current_layout().map.unwrap_or(Rect::new(1, 14, 66, 22))
}

pub fn contains_screen_cell(map: Rect, row: i32, col: i32) -> bool {
    row >= map.row && row < map.row + map.height && col >= map.col && col < map.col + map.width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classic_bounds_and_print_offsets_match_legacy_panel_formulas() {
        let viewport = calculate_viewport(Rect::new(1, 14, 66, 22), 2, 3, 66, 198);

        assert_eq!(
            viewport,
            Viewport {
                world_top: 23,
                world_bottom: 44,
                world_left: 100,
                world_right: 165,
                panel_row: 2,
                panel_col: 3,
                screen_row: 1,
                screen_col: 14,
                panel_row_prt: 21,
                panel_col_prt: 85,
                max_panel_rows: 4,
                max_panel_cols: 4,
            }
        );
    }

    #[test]
    fn classic_scroll_triggers_match_legacy_half_viewport_steps() {
        let map = Rect::new(1, 14, 66, 22);
        let (viewport, changed) = update_viewport(map, 2, 3, 66, 198, 42, 162, false);
        assert!(
            !changed,
            "positions inside the legacy margin should not scroll"
        );
        assert_eq!((viewport.panel_row, viewport.panel_col), (2, 3));

        let (viewport, changed) = update_viewport(map, 2, 3, 66, 198, 45, 169, false);
        assert!(changed, "crossing the legacy margin should scroll");
        assert_eq!((viewport.panel_row, viewport.panel_col), (3, 4));
        assert_eq!((viewport.world_top, viewport.world_left), (34, 133));
    }

    #[test]
    fn classic_scroll_matches_legacy_formulas_across_player_position_grid() {
        let map = Rect::new(1, 14, 66, 22);
        for y in 1..=66 {
            for x in 1..=198 {
                let current = calculate_viewport(map, 2, 3, 66, 198);
                let expected_row = if y < current.world_top + 2 || y > current.world_bottom - 2 {
                    ((y - 2) / 11).clamp(0, 4)
                } else {
                    2
                };
                let expected_col = if x < current.world_left + 3 || x > current.world_right - 3 {
                    ((x - 3) / 33).clamp(0, 4)
                } else {
                    3
                };
                let (updated, _) = update_viewport(map, 2, 3, 66, 198, y, x, false);

                assert_eq!(
                    (updated.panel_row, updated.panel_col),
                    (expected_row, expected_col)
                );
                assert_eq!(updated.panel_row_prt, updated.world_top - 2);
                assert_eq!(updated.panel_col_prt, updated.world_left - 15);
            }
        }
    }

    #[test]
    fn expanded_viewport_clamps_at_world_edges() {
        let map = Rect::new(1, 0, 81, 37);
        let (viewport, changed) = update_viewport(map, 0, 0, 66, 198, 66, 198, true);
        assert!(changed);
        assert_eq!((viewport.world_top, viewport.world_bottom), (30, 66));
        assert_eq!((viewport.world_left, viewport.world_right), (118, 198));
        assert_eq!((viewport.max_panel_rows, viewport.max_panel_cols), (2, 3));
    }

    #[test]
    fn expanded_viewport_scrolls_by_half_its_height() {
        let map = Rect::new(1, 0, 100, 40);
        let (viewport, changed) = update_viewport(map, 0, 0, 100, 200, 39, 3, false);

        assert!(changed);
        assert_eq!(viewport.panel_row, 1);
        assert_eq!(viewport.world_top, 21);
    }

    #[test]
    fn viewport_centers_world_smaller_than_map() {
        let viewport = calculate_viewport(Rect::new(1, 0, 81, 37), 0, 0, 20, 40);

        assert_eq!((viewport.world_top, viewport.world_bottom), (1, 20));
        assert_eq!((viewport.world_left, viewport.world_right), (1, 40));
        assert_eq!((viewport.screen_row, viewport.screen_col), (9, 20));
        assert_eq!((viewport.panel_row_prt, viewport.panel_col_prt), (-9, -20));
    }

    #[test]
    fn screen_cell_containment_includes_map_edges_only() {
        let map = Rect::new(1, 0, 81, 37);

        assert!(contains_screen_cell(map, 1, 0));
        assert!(contains_screen_cell(map, 37, 80));
        assert!(!contains_screen_cell(map, 0, 0));
        assert!(!contains_screen_cell(map, 38, 80));
        assert!(!contains_screen_cell(map, 37, 81));
    }
}
