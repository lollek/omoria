//! Screen geometry; coordinates are zero-based terminal cells.
//!
//! A panel renderer takes a game-state snapshot and returns fitted lines using
//! `render_lines`. Its terminal-facing wrapper draws them with:
//! ```ignore
//! draw_lines(rect, &lines, |row, col, line| {
//!     ncurses::mvaddstr(row, col, line);
//! });
//! ```

use std::sync::RwLock;

pub const CLASSIC_MIN_ROWS: i32 = 25;
pub const CLASSIC_MIN_COLS: i32 = 150;
pub const EQUIPMENT_HEIGHT: i32 = 14;
pub const INVENTORY_HEIGHT: i32 = 10;
pub const LOG_MIN_HEIGHT: i32 = 10;
pub const MONSTERS_MIN_HEIGHT: i32 = 5;
pub const FLOOR_ITEMS_MIN_HEIGHT: i32 = 5;
pub const SIDEBAR_WIDTH: i32 = 69;
pub const SPLIT_MIN_ROWS: i32 = {
    let log_bottom = CLASSIC_MIN_ROWS + LOG_MIN_HEIGHT;
    let sidebar_bottom =
        1 + EQUIPMENT_HEIGHT + INVENTORY_HEIGHT + MONSTERS_MIN_HEIGHT + FLOOR_ITEMS_MIN_HEIGHT;
    if log_bottom > sidebar_bottom {
        log_bottom
    } else {
        sidebar_bottom
    }
};
pub const SPLIT_MIN_COLS: i32 = CLASSIC_MIN_COLS;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rect {
    pub row: i32,
    pub col: i32,
    pub width: i32,
    pub height: i32,
}

impl Rect {
    pub const fn new(row: i32, col: i32, width: i32, height: i32) -> Self {
        Self {
            row,
            col,
            width,
            height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayoutKind {
    Classic,
    Split,
    TooSmall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Layout {
    pub kind: LayoutKind,
    pub prompt: Option<Rect>,
    pub map: Option<Rect>,
    pub stat_column: Option<Rect>,
    pub status_rows: Option<Rect>,
    pub equipment: Option<Rect>,
    pub log: Option<Rect>,
    pub inventory: Option<Rect>,
    pub monsters: Option<Rect>,
    pub floor_items: Option<Rect>,
}

pub fn compute_layout(rows: i32, cols: i32) -> Layout {
    let mut layout = Layout {
        kind: LayoutKind::TooSmall,
        prompt: None,
        map: None,
        stat_column: None,
        status_rows: None,
        equipment: None,
        log: None,
        inventory: None,
        monsters: None,
        floor_items: None,
    };
    if rows < CLASSIC_MIN_ROWS || cols < CLASSIC_MIN_COLS {
        return layout;
    }

    layout.kind = LayoutKind::Classic;
    layout.prompt = Some(Rect::new(0, 0, 81, 1));
    layout.map = Some(Rect::new(1, 14, 66, 22));
    layout.stat_column = Some(Rect::new(1, 0, 14, 22));
    layout.status_rows = Some(Rect::new(23, 0, 81, 2));
    layout.equipment = Some(Rect::new(1, 81, cols - 81, EQUIPMENT_HEIGHT));

    if rows >= SPLIT_MIN_ROWS && cols >= SPLIT_MIN_COLS {
        layout.kind = LayoutKind::Split;
        let sidebar_col = cols - SIDEBAR_WIDTH;
        let map_width = sidebar_col;
        let map_height = rows - 13;
        layout.stat_column = None;
        layout.map = Some(Rect::new(1, 0, map_width, map_height));
        layout.status_rows = Some(Rect::new(rows - 12, 0, map_width, 2));
        layout.log = Some(Rect::new(
            rows - LOG_MIN_HEIGHT,
            0,
            map_width,
            LOG_MIN_HEIGHT,
        ));
        let inventory_row = 1 + EQUIPMENT_HEIGHT;
        layout.equipment = Some(Rect::new(1, sidebar_col, SIDEBAR_WIDTH, EQUIPMENT_HEIGHT));
        layout.inventory = Some(Rect::new(
            inventory_row,
            sidebar_col,
            SIDEBAR_WIDTH,
            INVENTORY_HEIGHT,
        ));
        let monsters_row = inventory_row + INVENTORY_HEIGHT;
        let sidebar_height = rows - monsters_row;
        let extra_height = sidebar_height - MONSTERS_MIN_HEIGHT - FLOOR_ITEMS_MIN_HEIGHT;
        let monsters_height = MONSTERS_MIN_HEIGHT + (extra_height + 1) / 2;
        let floor_items_height = sidebar_height - monsters_height;
        layout.monsters = Some(Rect::new(
            monsters_row,
            sidebar_col,
            SIDEBAR_WIDTH,
            monsters_height,
        ));
        layout.floor_items = Some(Rect::new(
            monsters_row + monsters_height,
            sidebar_col,
            SIDEBAR_WIDTH,
            floor_items_height,
        ));
        debug_assert!(monsters_height >= MONSTERS_MIN_HEIGHT);
        debug_assert!(floor_items_height >= FLOOR_ITEMS_MIN_HEIGHT);
    }
    layout
}

lazy_static! {
    static ref CURRENT_LAYOUT: RwLock<Layout> =
        RwLock::new(compute_layout(CLASSIC_MIN_ROWS, CLASSIC_MIN_COLS));
}

pub fn current_layout() -> Layout {
    *CURRENT_LAYOUT.read().expect("current layout lock poisoned")
}

pub fn set_current_layout(layout: Layout) {
    *CURRENT_LAYOUT
        .write()
        .expect("current layout lock poisoned") = layout;
}

pub fn set_current_layout_for_dimensions(rows: i32, cols: i32) -> Layout {
    let layout = compute_layout(rows, cols);
    set_current_layout(layout);
    layout
}

#[cfg(test)]
pub fn reset_current_layout() {
    set_current_layout(compute_layout(CLASSIC_MIN_ROWS, CLASSIC_MIN_COLS));
}

/// Fit a panel's rendered snapshot to its rectangle, including blank rows.
/// Text is measured in characters; panel renderers should supply single-cell text.
pub fn render_lines(rect: Rect, lines: &[impl AsRef<str>]) -> Vec<String> {
    assert!(rect.width >= 0 && rect.height >= 0, "negative panel size");
    (0..rect.height as usize)
        .map(|row| {
            let mut line: String = lines
                .get(row)
                .map_or("", AsRef::as_ref)
                .chars()
                .take(rect.width as usize)
                .collect();
            line.extend(std::iter::repeat_n(
                ' ',
                rect.width as usize - line.chars().count(),
            ));
            line
        })
        .collect()
}

/// Draw a pure renderer's output through a terminal or test sink.
pub fn draw_lines(rect: Rect, lines: &[impl AsRef<str>], mut draw: impl FnMut(i32, i32, &str)) {
    for (offset, line) in render_lines(rect, lines).iter().enumerate() {
        if !line.is_empty() {
            draw(rect.row + offset as i32, rect.col, line);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn areas(layout: Layout) -> Vec<Rect> {
        [
            layout.prompt,
            layout.map,
            layout.stat_column,
            layout.status_rows,
            layout.equipment,
            layout.log,
            layout.inventory,
            layout.monsters,
            layout.floor_items,
        ]
        .iter()
        .filter_map(|rect| *rect)
        .collect()
    }

    fn overlaps(a: Rect, b: Rect) -> bool {
        a.row < b.row + b.height
            && b.row < a.row + a.height
            && a.col < b.col + b.width
            && b.col < a.col + a.width
    }

    #[test]
    fn classic_rects_match_existing_screen_positions() {
        let layout = compute_layout(25, 150);
        assert_eq!(layout.kind, LayoutKind::Classic);
        assert_eq!(layout.prompt, Some(Rect::new(0, 0, 81, 1)));
        assert_eq!(layout.map, Some(Rect::new(1, 14, 66, 22)));
        assert_eq!(layout.stat_column, Some(Rect::new(1, 0, 14, 22)));
        assert_eq!(layout.status_rows, Some(Rect::new(23, 0, 81, 2)));
        assert_eq!(layout.equipment, Some(Rect::new(1, 81, 69, 14)));
        assert_eq!(layout.log, None);
        assert_eq!(layout.inventory, None);
        assert_eq!(layout.monsters, None);
        assert_eq!(layout.floor_items, None);
    }

    #[test]
    fn undersized_terminals_have_no_drawable_areas() {
        for (rows, cols) in [(24, 150), (25, 149), (100, 149), (0, 0), (-1, 150)] {
            let layout = compute_layout(rows, cols);
            assert_eq!(layout.kind, LayoutKind::TooSmall);
            assert!(areas(layout).is_empty());
        }
    }

    #[test]
    fn split_selection_obeys_both_minimum_dimensions() {
        assert_eq!(
            compute_layout(SPLIT_MIN_ROWS - 1, SPLIT_MIN_COLS).kind,
            LayoutKind::Classic
        );
        assert_ne!(
            compute_layout(SPLIT_MIN_ROWS, SPLIT_MIN_COLS - 1).kind,
            LayoutKind::Split
        );
        for (rows, cols) in [
            (SPLIT_MIN_ROWS, SPLIT_MIN_COLS),
            (SPLIT_MIN_ROWS + 1, SPLIT_MIN_COLS),
            (SPLIT_MIN_ROWS, SPLIT_MIN_COLS + 1),
            (100, 250),
        ] {
            assert_eq!(compute_layout(rows, cols).kind, LayoutKind::Split);
        }
    }

    #[test]
    fn split_areas_fit_without_overlap_and_preserve_existing_drawers() {
        for rows in SPLIT_MIN_ROWS..=100 {
            for cols in SPLIT_MIN_COLS..=200 {
                let layout = compute_layout(rows, cols);
                let classic = compute_layout(CLASSIC_MIN_ROWS, cols);
                let map_width = cols - SIDEBAR_WIDTH;
                let sidebar_col = map_width;
                assert_eq!(layout.prompt, classic.prompt);
                assert_eq!(layout.map, Some(Rect::new(1, 0, map_width, rows - 13)));
                assert_eq!(layout.stat_column, None);
                assert_eq!(
                    layout.status_rows,
                    Some(Rect::new(rows - 12, 0, map_width, 2))
                );
                assert_eq!(
                    layout.equipment,
                    Some(Rect::new(1, sidebar_col, SIDEBAR_WIDTH, EQUIPMENT_HEIGHT))
                );
                assert_eq!(
                    layout.log,
                    Some(Rect::new(
                        rows - LOG_MIN_HEIGHT,
                        0,
                        map_width,
                        LOG_MIN_HEIGHT
                    ))
                );
                assert_eq!(
                    layout.inventory,
                    Some(Rect::new(15, sidebar_col, SIDEBAR_WIDTH, INVENTORY_HEIGHT))
                );
                assert_eq!(
                    layout.monsters,
                    Some(Rect::new(
                        25,
                        sidebar_col,
                        SIDEBAR_WIDTH,
                        MONSTERS_MIN_HEIGHT + ((rows - SPLIT_MIN_ROWS + 1) / 2),
                    ))
                );
                let monsters = layout.monsters.unwrap();
                let floor_items = layout.floor_items.unwrap();
                assert_eq!(
                    floor_items,
                    Rect::new(
                        monsters.row + monsters.height,
                        sidebar_col,
                        SIDEBAR_WIDTH,
                        rows - 25 - monsters.height,
                    )
                );
                assert!(monsters.height >= MONSTERS_MIN_HEIGHT);
                assert!(floor_items.height >= FLOOR_ITEMS_MIN_HEIGHT);

                let rects = areas(layout);
                for (index, rect) in rects.iter().enumerate() {
                    assert!(rect.row >= 0 && rect.col >= 0);
                    assert!(rect.width > 0 && rect.height > 0);
                    assert!(rect.row + rect.height <= rows);
                    assert!(rect.col + rect.width <= cols);
                    for other in &rects[index + 1..] {
                        assert!(!overlaps(*rect, *other), "{:?} overlaps {:?}", rect, other);
                    }
                }
            }
        }
    }

    #[test]
    fn split_sidebar_divides_monsters_area_and_shares_extra_rows() {
        assert_eq!(SPLIT_MIN_ROWS, 35);
        for (rows, monsters_height, floor_items_height) in
            [(35, 5, 5), (36, 6, 5), (37, 6, 6), (38, 7, 6)]
        {
            let layout = compute_layout(rows, SPLIT_MIN_COLS);
            let monsters = layout.monsters.unwrap();
            let floor_items = layout.floor_items.unwrap();

            assert_eq!(monsters.height, monsters_height);
            assert_eq!(floor_items.height, floor_items_height);
            assert_eq!(monsters.row + monsters.height, floor_items.row);
            assert_eq!(monsters.row, 25);
            assert_eq!(monsters.width, SIDEBAR_WIDTH);
            assert_eq!(floor_items.width, SIDEBAR_WIDTH);
            assert_eq!(floor_items.row + floor_items.height, rows);
        }
    }

    #[test]
    fn split_keeps_stat_and_status_rows_below_the_map() {
        let layout = compute_layout(SPLIT_MIN_ROWS, SPLIT_MIN_COLS);

        assert_eq!(layout.stat_column, None);
        assert_eq!(layout.status_rows, Some(Rect::new(23, 0, 81, 2)));
    }

    #[test]
    fn split_map_and_bottom_panels_fill_space_left_of_fixed_sidebar() {
        for (rows, cols) in [(35, 150), (50, 180)] {
            let layout = compute_layout(rows, cols);
            let map_width = cols - (CLASSIC_MIN_COLS - 81);
            let map_height = rows - 13;
            let sidebar_col = cols - (CLASSIC_MIN_COLS - 81);

            assert_eq!(layout.map, Some(Rect::new(1, 0, map_width, map_height)));
            assert_eq!(
                layout.status_rows,
                Some(Rect::new(rows - 12, 0, map_width, 2))
            );
            assert_eq!(layout.log, Some(Rect::new(rows - 10, 0, map_width, 10)));
            assert_eq!(layout.equipment, Some(Rect::new(1, sidebar_col, 69, 14)));
            assert_eq!(layout.inventory, Some(Rect::new(15, sidebar_col, 69, 10)));
            assert_eq!(
                layout.monsters,
                Some(Rect::new(
                    25,
                    sidebar_col,
                    69,
                    MONSTERS_MIN_HEIGHT + ((rows - SPLIT_MIN_ROWS + 1) / 2)
                ))
            );
            let monsters = layout.monsters.unwrap();
            assert_eq!(
                layout.floor_items,
                Some(Rect::new(
                    monsters.row + monsters.height,
                    sidebar_col,
                    69,
                    rows - 25 - monsters.height
                ))
            );
        }
    }

    #[test]
    fn renderer_truncates_and_pads_to_exact_rect_dimensions() {
        let rect = Rect::new(5, 8, 4, 3);
        assert_eq!(
            render_lines(rect, &["abcdef", "xy"]),
            vec!["abcd", "xy  ", "    "]
        );
        assert_eq!(
            render_lines(Rect::new(0, 0, 2, 1), &["one", "two"]),
            vec!["on"]
        );
    }

    #[test]
    fn renderer_does_not_split_utf8_characters() {
        assert_eq!(
            render_lines(Rect::new(0, 0, 3, 1), &["caf\u{e9}"]),
            vec!["caf"]
        );
        assert_eq!(
            render_lines(Rect::new(0, 0, 4, 1), &["caf\u{e9}"]),
            vec!["caf\u{e9}"]
        );
    }

    #[test]
    fn drawing_places_fitted_lines_at_the_rect_origin() {
        let mut calls = Vec::new();
        draw_lines(
            Rect::new(5, 8, 4, 3),
            &["abcdef", "xy"],
            |row, col, line| {
                calls.push((row, col, line.to_string()));
            },
        );
        assert_eq!(
            calls,
            vec![
                (5, 8, "abcd".to_string()),
                (6, 8, "xy  ".to_string()),
                (7, 8, "    ".to_string()),
            ]
        );
    }

    #[test]
    fn empty_rectangles_produce_no_drawing_calls() {
        for rect in [Rect::new(0, 0, 0, 3), Rect::new(0, 0, 3, 0)] {
            draw_lines(rect, &["text"], |_, _, _| {
                panic!("empty rectangles must not draw")
            });
        }
    }

    #[test]
    #[serial_test::serial]
    fn current_layout_can_be_set_and_reset_without_a_terminal() {
        reset_current_layout();
        assert_eq!(current_layout(), compute_layout(25, 150));
        set_current_layout(compute_layout(SPLIT_MIN_ROWS, SPLIT_MIN_COLS));
        assert_eq!(current_layout().kind, LayoutKind::Split);
        reset_current_layout();
        assert_eq!(current_layout(), compute_layout(25, 150));
    }

    #[test]
    #[serial_test::serial]
    fn terminal_resize_updates_layout_through_all_size_modes() {
        reset_current_layout();
        for (rows, cols, expected_kind) in [
            (24, 149, LayoutKind::TooSmall),
            (25, 150, LayoutKind::Classic),
            (35, 150, LayoutKind::Split),
            (25, 150, LayoutKind::Classic),
            (35, 150, LayoutKind::Split),
        ] {
            let layout = set_current_layout_for_dimensions(rows, cols);
            assert_eq!(layout.kind, expected_kind);
            assert_eq!(current_layout(), layout);
        }
        reset_current_layout();
    }
}
