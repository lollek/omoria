# Light Radius Extension — Implementation Plan

**Tracks:** GitHub issue #92 — *Extend line-of-sight to 2 or 3 squares*

---

## Goal

Replace the hardcoded `±1` torch-light radius with a `LIGHT_RADIUS` constant (starting at `1`,
designed to be bumped to `2` or `3`) and add a per-cell LOS guard so torchlight cannot
bleed through walls or closed doors.

---

## Current State (as-is)

| Location | What happens today |
|---|---|
| `src/dungeon/light.c` | Four sub-functions light/unlight a 3×3 box (`±1`) around the player. No LOS check — unnecessary at radius 1 because every cell is adjacent. |
| `src/spells.c` | Two teleport/darkness loops also use `±1` to clear `is_temporarily_lit`. |
| `src/player.h` / `src/player.c` | `bool player_light` declared. No radius field. |
| `src/misc.c` | `bool los(y1, x1, y2, x2)` already exists (~line 1379). Bresenham-style, checks `cave[y][x].fopen`. Used for monster visibility and spells. |

### Key observation

Because radius 1 makes every lit cell adjacent to the player, nothing can be "behind" a wall — so
no LOS check was ever needed. At radius ≥ 2 this is no longer true.

---

## Design Decisions

### 1. Constant vs. variable radius

Start with a **`const`** rather than a per-item runtime variable.  Reason: the issue asks for a
clean foundation first; item-specific radii can be added later without changing the architecture.

```rust
// src/data/constants.rs  (or a new src/dungeon/light_radius.rs)
pub const LIGHT_RADIUS: i64 = 1;   // bump to 3 when done
```

A C-accessible `#define` will expose it to the C side:

```c
// src/dungeon/light.h  (or a new header)
#define LIGHT_RADIUS 1
```

Until item-specific radii are needed, `LIGHT_RADIUS` is a compile-time constant.  When we want
items to differ, the natural next step is a `player_light_radius: i64` field alongside
`player_light`, following the same pattern as the issue description but staying as a const for now.

### 2. LOS guard strategy

* **Clearing** (`is_temporarily_lit = false`): No LOS check — always clear the full `±LIGHT_RADIUS`
  box unconditionally.  A stale lit cell is worse than a briefly unlit one.
* **Setting light** (`is_temporarily_lit = true`): Require `los(player_y, player_x, cell_y, cell_x)`
  per cell.
* **Wall tiles at the edge**: `los()` walks intermediate cells, not the target, so a wall at the
  perimeter of the radius will be correctly illuminated (you can see it) but won't allow light to
  pass through it.

### 3. Rust vs. C

The `los()` function is C today.  `light.c` is C today.  Porting all of `light.c` to Rust
is desirable but is a significant scope increase. **Plan:**

* Phase 1 (this issue): Keep `light.c` in C, add the constant and LOS guard in C.  This keeps the
  diff small and reviewable.
* Phase 2 (future): Port `light.c` → Rust once we have good test coverage of the new behaviour
  as a regression baseline.

The one exception: **unit tests** for the LOS-guard logic go in Rust via a thin Rust wrapper
that drives a small synthetic cave, allowing deterministic tests without spinning up ncurses.  See
Testing Strategy below.

### 4. Radius change mid-game (edge case)

If `LIGHT_RADIUS` is a const there is no mid-game change problem.  When item radii are added
later, the fix is: always clear a `MAX_LIGHT_RADIUS` box, not the current radius box.

### 5. Map bounds

The light loops near map edges can produce out-of-bounds indices.  Existing code already uses
`maxmin`/`minmax` for the *draw* block.  The light-set and light-clear loops **must** clamp to
`[1, MAX_HEIGHT-1]` × `[1, MAX_WIDTH-1]` (matching the existing cave array bounds convention).

---

## Files to Change

### Phase 1 — constant + LOS guard (C side)

| File | Change |
|---|---|
| `src/dungeon/light.h` | Add `#define LIGHT_RADIUS 1` |
| `src/dungeon/light.c` | Replace all `±1` loop bounds with `±LIGHT_RADIUS`; add `los()` guard to every "set light" loop; add bounds clamping to all loops |
| `src/spells.c` | Two teleport/darkness clear-loops: `±1` → `±LIGHT_RADIUS` |
| `src/player.h` / `src/player.c` | (Optional for Phase 1) Space reserved for `long player_light_radius` — not needed while using a const |
| `CHANGELOG.md` | Player-facing note |

### Detailed changes in `light.c`

#### `ml__draw_block`
```c
// Before
long const new_topp = y2 - 1;
long const new_bott = y2 + 1;
long const new_left = x2 - 1;
long const new_righ = x2 + 1;
// After
long const new_topp = y2 - LIGHT_RADIUS;
long const new_bott = y2 + LIGHT_RADIUS;
long const new_left = x2 - LIGHT_RADIUS;
long const new_righ = x2 + LIGHT_RADIUS;
```
(The `topp/bott/left/right` used for the outer draw loop already come from `y1/y2/x1/x2`, so
they naturally span both old and new positions — no change needed there.)

#### `ml__sub1_move_light` — Normal movement
```c
// Clear loop: ±1 → ±LIGHT_RADIUS, with bounds clamping
for (long i = max(1, y1 - LIGHT_RADIUS); i <= min(MAX_HEIGHT-1, y1 + LIGHT_RADIUS); i++)
  for (long j = max(1, x1 - LIGHT_RADIUS); j <= min(MAX_WIDTH-1, x1 + LIGHT_RADIUS); j++)
    cave[i][j].is_temporarily_lit = false;

// Set loop: ±LIGHT_RADIUS + los() guard + bounds clamping
for (long i = max(1, y2 - LIGHT_RADIUS); i <= min(MAX_HEIGHT-1, y2 + LIGHT_RADIUS); i++)
  for (long j = max(1, x2 - LIGHT_RADIUS); j <= min(MAX_WIDTH-1, x2 + LIGHT_RADIUS); j++)
    if (LIGHT_RADIUS <= 1 || los(y2, x2, i, j))
      cave[i][j].is_temporarily_lit = true;
```

Note the `LIGHT_RADIUS <= 1` short-circuit: when radius is 1 (today's default) the `los()` call
is skipped entirely — zero behaviour change for radius 1.

#### `ml__sub2_move_light` — Find/search mode
```c
// Clear loop: ±1 → ±LIGHT_RADIUS (bounds clamped)
for (long y = max(1, y1 - LIGHT_RADIUS); y <= min(MAX_HEIGHT-1, y1 + LIGHT_RADIUS); y++)
  for (long x = max(1, x1 - LIGHT_RADIUS); x <= min(MAX_WIDTH-1, x1 + LIGHT_RADIUS); x++)
    cave[y][x].is_temporarily_lit = false;

// Inspect loop (the inner y2/x2 loop): ±1 → ±LIGHT_RADIUS + bounds + los() on set
for (long y = max(1, y2 - LIGHT_RADIUS); y <= min(MAX_HEIGHT-1, y2 + LIGHT_RADIUS); y++)
  for (long x = max(1, x2 - LIGHT_RADIUS); x <= min(MAX_WIDTH-1, x2 + LIGHT_RADIUS); x++)
    // ... existing pwall/tptr logic, wrapped in los() guard for sets
```

#### `ml__sub3_move_light` — Blind mode
```c
// Clear loop only: ±1 → ±LIGHT_RADIUS (bounds clamped, no LOS needed)
for (long i1 = max(1, y1 - LIGHT_RADIUS); ...; i1++)
  for (long i2 = max(1, x1 - LIGHT_RADIUS); ...; i2++)
    cave[i1][i2].is_temporarily_lit = false;
```

#### `ml__sub4_move_light` — No light
```c
// Clear loop: same bounds expansion as sub3
for (long i1 = max(1, y1 - LIGHT_RADIUS); ...; i1++)
  for (long i2 = max(1, x1 - LIGHT_RADIUS); ...; i2++)
    cave[i1][i2].is_temporarily_lit = false;
    // ... existing test_light/lite_spot/unlite_spot logic unchanged
```

#### `spells.c` teleport clear (~line 728) and darkness clear (~line 2921)
```c
// Same pattern: ±1 → ±LIGHT_RADIUS with bounds clamping
for (long i1 = max(1, char_row - LIGHT_RADIUS); i1 <= min(MAX_HEIGHT-1, char_row + LIGHT_RADIUS); i1++)
  for (long i2 = max(1, char_col - LIGHT_RADIUS); i2 <= min(MAX_WIDTH-1, char_col + LIGHT_RADIUS); i2++)
    cave[i1][i2].is_temporarily_lit = false;
```

---

## Testing Strategy

### Why Rust tests?

Testing C with global cave state is painful and non-deterministic.  Instead:

1. Write a **Rust integration helper** in `src/dungeon/` (or `tests/`) that:
   * Allocates a small synthetic `cave` array (e.g. 10×10).
   * Populates `fopen` flags to model walls and open space.
   * Calls the light C functions via FFI.
   * Asserts `is_temporarily_lit` flags afterwards.

   However, FFI into C with global state is still messy.  A cleaner path:

2. **Extract the LOS-guard logic into a pure Rust function** in `src/dungeon/light.rs`:

```rust
// src/dungeon/light.rs
/// Returns true if cell (cy, cx) should be lit given player at (py, px),
/// checking LOS if radius > 1.
pub fn should_light_cell(
    py: i64, px: i64,
    cy: i64, cx: i64,
    radius: i64,
    is_open: impl Fn(i64, i64) -> bool,
) -> bool {
    if radius <= 1 {
        return true;  // at radius 1, all adjacent cells are lit unconditionally
    }
    bresenham_los(py, px, cy, cx, &is_open)
}

fn bresenham_los(y1: i64, x1: i64, y2: i64, x2: i64, is_open: &impl Fn(i64, i64) -> bool) -> bool {
    // port of misc.c los(), but pure — takes is_open closure instead of global cave[]
}
```

This function is:
* **Pure** — no global state, takes a closure for cell openness.
* **Testable** without C FFI or ncurses.
* Callable from C via an `extern "C"` wrapper in `src/dungeon/interop.rs` (or
  `src/dungeon/light_extern.rs`), following the existing FFI patterns.

The C `light.c` `ml__sub1_move_light` set-loop then calls this instead of `los()`:

```c
// In light.c (after Rust shim is in place)
if (rust_should_light_cell(y2, x2, i, j, LIGHT_RADIUS))
  cave[i][j].is_temporarily_lit = true;
```

### Test cases (TDD — RED first)

| # | Setup | Expected |
|---|---|---|
| T1 | radius=1, 3×3 open cave, player at (5,5) | All 9 cells in ±1 box have `is_temporarily_lit=true` |
| T2 | radius=2, open cave | All 25 cells in ±2 box lit |
| T3 | radius=2, wall at (5,7) with `fopen=false`, player at (5,5), cell (5,8) behind wall | `(5,8)` NOT lit; `(5,7)` IS lit (visible wall) |
| T4 | radius=2, cell (5,7) is a wall (`fopen=false`), cells diagonal to it | diagonal path clear? Check `should_light_cell` for diagonals |
| T5 | clear operation | All cells in ±radius box unconditionally cleared regardless of walls |
| T6 | radius=1 (current default) | Identical to current behaviour — T1 still passes |

---

## Phases (TDD)

### Phase 1-RED: Failing Rust tests for `should_light_cell`

1. Create `src/dungeon/light.rs` with `should_light_cell` stub returning `true` always.
2. Write test module with T1–T6 above (T3, T4 will fail with the stub).
3. **Stop. Navigator reviews.**

### Phase 1-GREEN: Implement `should_light_cell` + `bresenham_los`

1. Port `los()` from `misc.c` into `bresenham_los` in `light.rs` (pure Rust, no globals).
2. Implement `should_light_cell` calling it.
3. All T1–T6 pass.
4. Add `extern "C"` shim so C code can call it.
5. **Stop. Navigator reviews.**

### Phase 2-RED: Failing test/smoke for C `light.c` changes

*(Deferred — confirm approach with navigator first.)*

For the C changes we cannot easily write pure unit tests.  Options:

* **Option A**: Run the game and manually verify at radius 2 (smoke test).
* **Option B**: Write a C test harness with a synthetic cave (complex but possible).
* **Option C**: Accept the Rust `should_light_cell` tests as sufficient coverage for the LOS logic,
  and treat the C loop-bound changes as mechanical (review-only).

Recommended: **Option C** for now.

### Phase 2-GREEN: C `light.c` and `spells.c` changes

1. Add `#define LIGHT_RADIUS 1` to `src/dungeon/light.h`.
2. Replace all `±1` with `±LIGHT_RADIUS` in `light.c` (four sub-functions + `ml__draw_block`).
3. Add bounds clamping to all loops.
4. Replace `los()` call in set-loops with `rust_should_light_cell()` (or keep `los()` with the
   `LIGHT_RADIUS <= 1` short-circuit).
5. Update `spells.c` two clear-loops.
6. Update `CHANGELOG.md`.
7. Run `cargo test` + manual smoke test.
8. **Stop. Navigator reviews.**

### Phase 3-REFACTOR: Cleanup

1. Extract "clear box with clamping" into a helper `ml__clear_light_box(y, x, radius)`.
2. Extract "set box with clamping + LOS" into `ml__set_light_box(y, x, radius)`.
3. Reduce duplication across the four sub-functions.
4. `cargo test` still green, no new warnings.

---

## Risk Register

| Risk | Mitigation |
|---|---|
| `los()` semantics differ slightly from needed light semantics | Use Rust port with explicit tests; verify diagonal edge cases |
| Off-by-one on wall visibility (wall at edge of radius may not be visible) | T3 explicitly tests this — wall at perimeter must be lit but not pass-through |
| Out-of-bounds array access at map edges | Bounds clamping on all loops, tested near (1,1) and (MAX_HEIGHT-1, MAX_WIDTH-1) |
| `ml__draw_block` margins only expanded by 1 (the `±1` new_topp/bott/left/righ) | Expand these to `±LIGHT_RADIUS` to ensure newly lit cells at radius>1 are redrawn |
| `is_temporarily_lit` stale if radius changes mid-game | Not relevant while radius is a const; document for future item-radius work |
| `ml__sub2_move_light` complex rendering logic | Keep existing render logic, only change loop bounds and add LOS guard on set operations |

---

## Open Questions for Navigator

1. **Radius value**: Start at `LIGHT_RADIUS = 1` (no visual change), then bump to `3` in a second
   commit once tests pass?  Or go straight to `3`?
2. **Rust shim approach**: Port `los()` to pure Rust (`should_light_cell`) now, or keep using the
   C `los()` in `light.c` and only test the bounds/LOS at the C level?
3. **`ml__sub2_move_light`** has complex inline rendering mixed with the light-state logic.  For
   the Green phase, touch only the bounds and LOS guard; leave the rendering as-is.  Agree?
4. **`main_loop.c`**: Issue mentions optionally setting `light_radius` based on torch/lantern type.
   Defer to a later issue?
