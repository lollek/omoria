//! Pure Rust light-of-sight logic for torch illumination.
//!
//! Provides a cell-level LOS predicate (`should_light_cell`) used by the
//! C `light.c` move-light functions when `LIGHT_RADIUS > 1`.
//!
//! All functions here are **pure** — they take an `is_open` closure instead
//! of reading the global `cave[]` array directly, which makes them testable
//! without C FFI or ncurses.

/// Returns `true` if the cell at `(target_row, target_col)` should be lit
/// given the player standing at `(player_row, player_col)` with a light of the
/// given `light_radius`.
///
/// * When `radius <= 1`, always returns `true` — every adjacent cell is
///   reachable without anything in the way, so no LOS check is needed and we
///   preserve the current behaviour exactly.
/// * When `radius > 1`, delegates to `bresenham_los`.  The `is_open` closure
///   must return `true` for traversable cells (mirrors `cave[y][x].fopen`).
pub fn should_light_cell(
    player_row: i64,
    player_col: i64,
    target_row: i64,
    target_col: i64,
    light_radius: i64,
    is_open: impl Fn(i64, i64) -> bool,
) -> bool {
    if light_radius <= 1 {
        return true;
    }

    bresenham_los(player_row, player_col, target_row, target_col, &is_open)
}

/// Pure Rust port of the Bresenham-style `los()` from `src/misc.c` (~line 1379).
///
/// Returns `true` if there is no opaque cell between
/// `(source_row, source_col)` and `(target_row, target_col)`.
/// The check walks from `(target_row, target_col)` toward
/// `(source_row, source_col)`, testing intermediate
/// cells — **not** the source, and **not** the destination.  This means a wall
/// tile at the edge of the radius is visible (returns `true`) but blocks
/// anything behind it.
///
/// `is_open` is queried for each cell along the path; it should return `false`
/// for walls, closed doors, and other opaque tiles.
pub(crate) fn bresenham_los(
    source_row: i64,
    source_col: i64,
    mut target_row: i64,
    mut target_col: i64,
    is_open: &impl Fn(i64, i64) -> bool,
) -> bool {
    let mut clear_path = true;

    let delta_row = source_row - target_row;
    let delta_col = source_col - target_col;

    if delta_row != 0 || delta_col != 0 {
        let row_step = if delta_row < 0 { -1 } else { 1 };
        let col_step = if delta_col < 0 { -1 } else { 1 };

        if delta_row == 0 {
            loop {
                target_col += col_step;
                clear_path = is_open(target_row, target_col);
                if source_col == target_col || !clear_path {
                    break;
                }
            }
        } else if delta_col == 0 {
            loop {
                target_row += row_step;
                clear_path = is_open(target_row, target_col);
                if source_row == target_row || !clear_path {
                    break;
                }
            }
        } else if delta_row.abs() > delta_col.abs() {
            let slope = (delta_col.abs() as f64 / delta_row.abs() as f64) * col_step as f64;
            let mut scan_col = target_col as f64;

            loop {
                target_row += row_step;
                scan_col += slope;

                let col1 = (scan_col - 0.1 + 0.5) as i64;
                let col2 = (scan_col + 0.1 + 0.5) as i64;

                if !(is_open(target_row, col1) || is_open(target_row, col2)) {
                    clear_path = false;
                }

                if source_row == target_row || !clear_path {
                    break;
                }
            }
        } else {
            let slope = (delta_row.abs() as f64 / delta_col.abs() as f64) * row_step as f64;
            let mut scan_row = target_row as f64;

            loop {
                target_col += col_step;
                scan_row += slope;

                let row1 = (scan_row - 0.1 + 0.5) as i64;
                let row2 = (scan_row + 0.1 + 0.5) as i64;

                if !(is_open(row1, target_col) || is_open(row2, target_col)) {
                    clear_path = false;
                }

                if source_col == target_col || !clear_path {
                    break;
                }
            }
        }
    }

    clear_path
}

// ─── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn always_open(_row: i64, _col: i64) -> bool {
        true
    }

    /// Returns a closure that treats exactly `(wall_row, wall_col)` as closed.
    fn wall_at(wall_row: i64, wall_col: i64) -> impl Fn(i64, i64) -> bool {
        move |row, col| !(row == wall_row && col == wall_col)
    }

    // ── should_light_cell ────────────────────────────────────────────────────

    /// At radius 1 the fast-path fires: adjacent cells are always lit,
    /// regardless of whether the cell itself is "open".
    #[test]
    fn radius_1_always_lights_adjacent_cell() {
        assert!(
            should_light_cell(5, 5, 5, 6, 1, always_open),
            "adjacent cell with clear path should be lit"
        );
    }

    /// At radius 1 the fast-path should skip the LOS check entirely, so even
    /// a "wall" at the target is lit without inspecting is_open.
    #[test]
    fn radius_1_skips_los_check_for_wall_cell() {
        // wall_at(5,6) makes (5,6) closed — but radius=1 must still light it
        assert!(
            should_light_cell(5, 5, 5, 6, 1, wall_at(5, 6)),
            "radius 1 should light the cell unconditionally"
        );
    }

    /// Radius 2, straight open path — cell two steps away must be lit.
    #[test]
    fn radius_2_lights_cell_with_clear_path() {
        assert!(
            should_light_cell(5, 5, 5, 7, 2, always_open),
            "clear horizontal path at radius 2 should be lit"
        );
    }

    /// Radius 2: a wall at (5,6) blocks the cell at (5,7) behind it.
    /// This is the key behaviour that the stub FAILS — the RED test.
    #[test]
    fn radius_2_does_not_light_cell_behind_wall() {
        assert!(
            !should_light_cell(5, 5, 5, 7, 2, wall_at(5, 6)),
            "cell behind a wall should not be lit"
        );
    }

    /// Radius 2: the wall tile at the *edge* of the radius is itself visible
    /// (you can see it), even though it blocks cells further away.
    #[test]
    fn radius_2_wall_at_edge_is_visible() {
        // wall_at(5,7) — the path from (5,5) to (5,7) only walks (5,6),
        // which is open, so (5,7) is reachable.
        assert!(
            should_light_cell(5, 5, 5, 7, 2, wall_at(5, 7)),
            "wall at the edge of the radius should itself be lit"
        );
    }

    /// The player's own cell is always lit regardless of radius or walls.
    #[test]
    fn player_own_cell_is_always_lit() {
        assert!(
            should_light_cell(5, 5, 5, 5, 2, always_open),
            "player cell should always be lit"
        );
    }

    // ── bresenham_los ────────────────────────────────────────────────────────

    /// Same cell → always clear (no intermediate cells to check).
    #[test]
    fn los_same_cell_is_clear() {
        assert!(bresenham_los(5, 5, 5, 5, &always_open));
    }

    /// Horizontal clear path.
    #[test]
    fn los_horizontal_clear_path() {
        assert!(bresenham_los(5, 5, 5, 8, &always_open));
    }

    /// Horizontal path blocked by a wall at an intermediate cell.
    #[test]
    fn los_horizontal_blocked_by_wall() {
        // Wall at (5,7) should block the path from (5,5) to (5,8).
        assert!(!bresenham_los(5, 5, 5, 8, &wall_at(5, 7)));
    }

    /// Vertical clear path.
    #[test]
    fn los_vertical_clear_path() {
        assert!(bresenham_los(5, 5, 8, 5, &always_open));
    }

    /// Vertical path blocked by a wall.
    #[test]
    fn los_vertical_blocked_by_wall() {
        assert!(!bresenham_los(5, 5, 8, 5, &wall_at(7, 5)));
    }

    /// The wall at the *target* cell does not block LOS to that cell
    /// (los checks intermediate cells, not the destination itself).
    #[test]
    fn los_wall_at_destination_still_visible() {
        // (5,8) is a wall; the path (5,5)→(5,8) must still be "clear" to
        // (5,8) so the wall tile gets illuminated.
        assert!(bresenham_los(5, 5, 5, 8, &wall_at(5, 8)));
    }
}
