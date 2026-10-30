# Epic MC: Map Colors

A feature epic, not a port. It colors the map and the visible-monsters
sidebar:

* Cave walls, floors, corridors, and town buildings get terrain colors.
* Doors, stairs, rubble, traps, and shop entrances keep semantic fixture colors.
  Portable items use subtype colors, overridden by meaningful quality and
  special attributes.
* Every monster template carries its own color, used on the map, in spell
  and missile animations, and in the sidebar.
* The palette uses 256 colors where the terminal has them and falls back to
  the 8 basic colors (plus bold) elsewhere.

Back to the [backlog](../backlog.md).

Every story here changes what the player sees, so each one adds a
`CHANGELOG.md` bullet unless its card says it preserves behavior. Backlog
rules still apply: keep every C ABI and test the logic without a terminal.

## Current Flow

Checked against source on 2026-10-30.

| Piece | Today | Source |
| --- | --- | --- |
| Color setup | `start_color()` and six pairs (red, green, yellow, blue, magenta, cyan) on black. The pair number equals the `COLOR_*` constant. No `has_colors()` or `COLORS()` check. | `init_curses` in [ncurses.rs](../../../src/ncurses.rs) |
| Pair helper | `color_pair(pair)` wraps `COLOR_PAIR` | [ncurses.rs](../../../src/ncurses.rs) |
| Map glyph | `loc_symbol` returns one `chtype` per cell. Only water is colored (`` '`' `` with blue). Walls are `#`, floors `.`, unknown terrain `.` with `A_DIM`. | [misc.c](../../../src/misc.c) |
| Terrain | `cave[y][x].fval` from `enum floor_types`; sets `water_set`, `earth_set`, `pwall_set` | [floor.h](../../../src/floor.h), [variables.c](../../../src/variables.c) |
| Town buildings | `gc__build_store` fills the building with `ft_boundry_wall`, the same value as the town and dungeon border | [town.c](../../../src/generate_map/town.c) |
| Item glyph | `symbol()` returns the plain `chtype`; `C_item_get_glyph` adds the current ItemType-based role. Doors, stairs, rubble, traps, and shop entrances are `ItemType` values. Shop entrances already add `A_BOLD`. MC3 will replace portable-item category colors with semantic subtype/quality colors while leaving fixture roles unchanged. | [item_type.rs](../../../src/data/item_type.rs), [data/interop.rs](../../../src/data/interop.rs) |
| Item glyph as text | `C_item_get_tchar` remains plain; `inven.c` formats the glyph with `%c` and compares it as `char` | [model_item.h](../../../src/model_item.h), [inven.c](../../../src/inventory/inven.c) |
| Monster glyph | `monster_template_get_symbol` returns a plain `char`. `MonsterTemplate` has no color. About 390 literals in `data.rs`. | [monster_template/](../../../src/generate_monster/monster_template/) |
| Direct monster prints | These call `print()` with the plain symbol, bypassing `loc_symbol` | `creature.c`, `spells.c` |
| Missile item glyph | Thrown missiles use `C_item_get_glyph`, the same colored-glyph path as map items | [ranged.c](../../../src/combat/ranged.c), [data/interop.rs](../../../src/data/interop.rs) |
| Sidebar monsters | `render_monsters_panel` builds `"{symbol} {name} ({count})"` strings. `draw_lines` hands the sink a `&str` with no attributes. | [sidebar.rs](../../../src/user_interface/sidebar.rs), [layout.rs](../../../src/user_interface/layout.rs) |
| Tests | HUD `stat_color` test; file-backed `TestScreen` tests in `ncurses.rs`; `get_loc_symbol` checks in `tests/headless_turn.c` (only for `@`). None checks map colors. | |

The constraints the stories depend on:

* **Glyphs are `chtype`.** Attribute bits already travel with the character
  through `loc_symbol`, `print`, and `print_chstr` (shop entrances use
  `A_BOLD`). Callers that need only the character must mask it:
  `files.c` already uses `& 0x7F`.
* **`C_item_get_tchar` feeds text.** It remains plain for inventory prompts;
  map and missile drawing use `C_item_get_glyph` instead.
* **Matching code needs the plain symbol.** `spells.c` compares a monster
  symbol to a typed character and `files.c` prints it in the monster
  recall. These keep `monster_template_get_symbol`.
* **Pair numbers 1-6 are taken** by the HUD (`stat_color` and status
  fields). New pairs start at 16.
* **Two functions edit the same file.** MC2, MC3, and MC4 all change
  `loc_symbol` in `misc.c`, so they run in order.
* **Hallucination and blindness** are handled before the glyph is chosen
  (`loc_symbol` returns a space when blind), so colors never leak through.
* **No remembered tiles.** `get_loc_symbol` returns a space for cells that
  are not currently lit, so there is nothing to dim.

## Target Design

The shape the stories build:

* **Semantic roles, not color numbers.** A `MapColor` enum names each role
  (for example `Water`, `Granite`, `Magma`, `Quartz`, `BoundaryWall`,
  `RoomFloor`, `Corridor`, `Door`, `Stairs`, `Rubble`, `Trap`, `ShopEntrance`,
  `Gold`, `Potion`, `Scroll`, `Weapon`, `Armor`, and one per monster
  color). Game code asks for a role and receives an attribute.
* **One table, two palettes.** Each role maps to a 256-color index and an
  8-color fallback (color plus optional bold). `init_map_colors(color_count)`
  picks the column from `COLORS()` and registers one pair per role from pair
  16 up. With no color support the attribute is empty.
* **Pure mappings.** Terrain and full-item data resolve to semantic roles;
  role-to-palette mapping is also pure. These are L0 tests with no curses
  calls.
* **C asks through small shims.** `C_item_get_glyph` returns a `chtype` with
  attributes; `C_item_color_key` exposes the terminal-independent semantic
  key for stack compatibility. C never sees pair numbers.
* **Monsters own their color.** `MonsterTemplate.color` is data, set per
  monster. Map, animations, and sidebar read the same field.
* **The sidebar draws cells, not just strings.** The monsters panel draws the
  symbol with its attribute, then the rest of the line plain.

## Decisions

All decided on 2026-10-30.

### MCD1. Palette

Decision: 256-color with an 8-color fallback, chosen at startup from
`COLORS()`.

### MCD2. Monster Colors

Decision: a color field on every `MonsterTemplate`, assigned per monster,
not derived from the symbol letter.

### MCD3. Scope

Decision: walls, floors and corridors, town buildings and shop entrances,
doors, stairs, rubble, traps, monsters, and portable items. Portable items
use subtype colors with quality and special-attribute overrides; fixtures keep
their semantic roles.

### MCD4. Color Option

Decision: no on/off setting. Terminals without color support get no color
attributes.

### MCD5. Sidebar

Decision: the visible-monsters panel uses the same colors as the map.

### MCD6. Item Color Meaning

Decision: portable-item colors are semantic, not random. Resolve one role in
priority order: special attribute or explicit special-attribute combination,
quality, subtype, then the existing family role. A combination selects one
role through game logic; terminal color attributes are never combined. Colors
reflect actual item properties even before identification.

### MCD7. Item Color and Stack Identity

Decision: derive the semantic color key from existing item fields (subtype,
flags, and modifiers). Items with different keys do not merge into a stack;
copies and split stacks retain their key. No random seed, `Item`/`treasure_type`
field, C struct-layout change, or save migration is needed. Stack comparison is
based on the semantic key, even if a limited terminal palette renders two keys
with the same visible basic color.

### MCD8. Item Palette Budget

Decision: start with eight additional item color slots at pairs 48-55 and
reserve pairs 48-63 for future expansion. Multiple semantic reasons may use
the same slot. Preserve existing roles and pair numbers; 8-color fallbacks
may be visually shared.

## Story Graph

```mermaid
graph LR
  MC1[MC1 palette] --> MC2[MC2 terrain]
  MC1 --> MC3[MC3 items and fixtures]
  MC1 --> MC4[MC4 monsters]
  MC2 --> MC3
  MC3 --> MC4
  MC4 --> MC5[MC5 sidebar monsters]
```

Ordering notes:

* MC1 is the pathfinder. MC2 to MC5 use its role table and attribute
  lookup without changing their signatures.
* MC2, MC3, and MC4 all edit `loc_symbol` in [misc.c](../../../src/misc.c)
  and run in that order, never at the same time. Their other files are
  disjoint, so the Rust halves can be prepared in parallel and only the
  `misc.c` hook-up waits.
* MC5 follows MC4 because it reads `MonsterTemplate.color`.

## Stories

### MC1. Palette and Attribute Lookup (pathfinder: F-PALETTE)

Status: done (2026-10-30). Depends on: none.
Size: M. Complexity: Medium. Agent: strong.

Implementation: `MapColor` (32 roles, including monster colors) and the role
table are in [map_colors.rs](../../../src/user_interface/map_colors.rs);
`init_curses` calls `init_map_colors`. Water in `loc_symbol` now uses the new
`C_map_color_water_attr` shim (`map_colors.h`, `map_colors_extern.rs`), the
only `misc.c` change. Water uses ANSI blue (index 4) so 256-color terminals
match today's look. Verification: L0 and L1 tests (`cargo test --lib
map_colors`, `ncurses::tests`). Unverified: C-level `loc_symbol` water output
(left to MC2's L2 tests); real-terminal look.

Owns: [ncurses.rs](../../../src/ncurses.rs), a new
`src/user_interface/map_colors.rs` for the role table (and its module
registration). Must not touch: `misc.c`, item and monster data, sidebar.

Behavior:
* Add the `MapColor` role enum and the role table with a 256-color index and
  an 8-color fallback (color and bold) per role.
* Add `init_map_colors(color_count)` and `map_color_attr(role) -> chtype`.
  Pairs start at 16 and use a black background. Existing pairs 1-6 are
  untouched.
* Call it from `init_curses` after `start_color()`. If `has_colors()` is
  false, `map_color_attr` returns an empty attribute.
* Re-express water as `MapColor::Water` with the same visible result on an
  8-color terminal.

Foundation: F-PALETTE.
* `MapColor`, the role table, `init_map_colors`, and `map_color_attr`.

Acceptance checks:
* L0: every role has a 256-color value and a fallback; no fallback is black
  on black.
* L0: 256-color and 8-color selection from a color count; no color support
  gives an empty attribute.
* L1, file-backed `newterm()` with an 8-color and a 256-color terminal type:
  `COLORS()` matches, and an attribute written with `map_color_attr` reads
  back with the same pair through `mvinch`.
* Changelog: none (water keeps today's look).

Out of scope: any `loc_symbol` change except the water re-expression.

### MC2. Terrain Colors (fan-out)

Status: done (2026-10-30). Depends on: MC1.
Size: S. Complexity: Low. Agent: standard.

Owns: a new terrain-role function and its extern shim (module
registration in `lib.rs` or the owning `mod.rs`), and the wall, floor, and
water branches of `loc_symbol` in [misc.c](../../../src/misc.c). Must not
touch: item and monster branches of `loc_symbol` (MC3, MC4), palette
internals (MC1).

Behavior:
* Map `fval` to a role: granite, magma, and quartz walls each their own;
  boundary wall one color (town buildings and the border share it); room
  floor and corridor floor subtle, distinct colors; water as today.
* `loc_symbol` ORs the terrain attribute into the `#` and `.` results. Unknown
  terrain stays `.` with `A_DIM`.
* Glyph characters do not change.

Acceptance checks:
* L0: every `fval` in `pwall_set`, `earth_set`, and `water_set` maps to the
  expected role; an unknown `fval` maps to no role.
* L2, C headless: `get_loc_symbol` for a granite wall, a magma vein, a floor,
  and a boundary wall returns the expected character in the low byte and the
  expected pair bits, and still returns `@` for the player.
* Changelog entry: cave walls, floors, and town buildings are colored.

Implementation: `terrain_color(FloorType)` maps the existing typed terrain
variants to semantic roles. `C_map_color_terrain_attr` converts the C byte with
`FloorType::try_from` at the FFI boundary, keeping raw floor values out of the
role mapping. The shim supplies attributes to `loc_symbol` for wall and floor
glyphs; water keeps its existing shim and appearance. `C_map_color_init` shares
palette initialization with C callers. Unknown terrain and glyph characters
are unchanged.

Verification: `cargo test --lib map_colors` covers every modeled terrain value
in the wall, earth, and water sets. The file-backed 8-color and 256-color tests
verify typed terrain attributes against registered role pairs; `make test-headless`
verifies C glyph bytes and expected pair bits with a file-backed 256-color
screen, including the hidden-monster terrain fallback and player glyph.
`CARGO_TERM_QUIET=true make -s check` passed (933 Rust tests and all available
C/headless checks).

Unverified: real-terminal visual appearance; `test-traps` requires Linux GNU
ld `--wrap` and was skipped on macOS.

Out of scope: items, doors, and monsters.

### MC3. Items and Fixtures (fan-out)

Status: open (revised 2026-10-30). Depends on: MC1, MC2 (for `misc.c` order).
Size: L. Complexity: High. Agent: standard.

Owns: the pure semantic color resolver for full `Item` data (new
`src/data/item_color.rs`), item palette roles in
[map_colors.rs](../../../src/user_interface/map_colors.rs),
`C_item_get_glyph` and `C_item_color_key` in
[data/interop.rs](../../../src/data/interop.rs) and their declarations,
portable-item branches of `loc_symbol`, the missile print in
[ranged.c](../../../src/combat/ranged.c), and the stack-merge predicate in
[inven.c](../../../src/inventory/inven.c). Must preserve the plain behavior of
`C_item_get_tchar` and inventory text formatting; must not touch monster code.

Behavior:
* Resolve portable-item color from the full item: special attributes or
  explicit combinations first, then quality, subtype, and the current family
  role as fallback. Use flags and modifiers that survive item copies and saves;
  do not rely on transient `ItemQuality` or random per-instance seeds.
* Resolve multiple special attributes to one documented semantic role. Do not
  combine terminal color attributes. Use actual item properties even when the
  item is unidentified.
* Keep doors, stairs, rubble, seen traps, shop entrances, and other map
  fixtures on their existing semantic roles. Unseen traps and secret doors
  retain their current glyphs and remain uncolored.
* `C_item_get_glyph` uses the resolved item role for map and missile drawing;
  `C_item_get_tchar` remains unchanged for text callers.
* `C_item_color_key` exposes the terminal-independent role to C inventory
  stacking. Otherwise-stackable items merge only when their semantic keys
  match; differently colored keys stay in separate stacks. Splitting copies
  the key, so both resulting stacks retain the same color.
* Add eight item palette slots initially, starting at pair 48. Keep existing
  role and pair assignments stable; reserve pairs through 63 for later item
  colors. Some semantic keys may share a palette slot, especially on 8-color
  terminals.
* Derive color from existing fields. Do not add a per-item color/seed field,
  change the `Item`/`treasure_type` layout, or require save migration.

Acceptance checks:
* L0: exhaustive portable `ItemType` fallback coverage; table-driven subtype,
  quality, special-attribute, and combined-attribute cases resolve to the
  expected single role, including precedence and unknown-value fallback.
* L0: color resolution is deterministic for the same item fields and does not
  depend on identification state; a quality override can distinguish items
  that share a broad `ItemType`.
* L0: `C_item_get_tchar` remains plain and unchanged for every renderable
  `ItemType`; fixture glyph roles and shop bold remain unchanged.
* L1, C inventory boundary: otherwise-stackable items with the same semantic
  key merge; different keys remain separate; splitting preserves the key.
* L2, C headless: portable-item map glyphs return expected characters and
  palette pairs; doors, stairs, rubble, traps, shops, unseen traps, and secret
  doors retain their expected glyph and color behavior. Map and missile paths
  use the same resolved key.
* Changelog entry: portable item colors communicate subtype, quality, and
  special attributes; stack behavior follows the same semantic color key.

Out of scope: inventory and equipment text, item names, MC4/MC5, random
per-instance colors, and changes to the persisted item layout.

The category-only MC3 implementation was completed and verified earlier, but
is reopened because it gives every item in a broad type the same color and
does not make stack merging aware of color semantics. Its prior tests and full
check are historical evidence only; they do not satisfy this revised story.

Verification for the revised behavior is pending. Re-run the acceptance checks
above, `make check-fast`, and `CARGO_TERM_QUIET=true make -s check` after
implementation. Real-terminal appearance remains a manual check; on macOS,
`test-traps` is skipped because GNU `ld --wrap` is unavailable.

### MC4. Monster Colors (fan-out)

Status: open. Depends on: MC1, MC3 (for `misc.c` order).
Size: L. Complexity: Medium. Agent: standard.

Owns: [monster_template/](../../../src/generate_monster/monster_template/)
(`template.rs`, `data.rs`, `interop.rs`, tests),
`monster_template.h`, the monster branch of `loc_symbol`, and the direct
`print()` calls in [creature.c](../../../src/creature.c) and
[spells.c](../../../src/spells.c). Must not touch: sidebar (MC5),
`monster_template_get_symbol` callers that match or print the character.

Behavior:
* Add `color: MapColor` to `MonsterTemplate`; update `GLITCH_TEMPLATE`, the
  test helper, and every literal in `data.rs`. Assign a color per monster
  and review the assignments by hand.
* Add `monster_template_get_glyph(index) -> chtype` (symbol plus color) and
  declare it in `monster_template.h`.
* `loc_symbol` and the direct prints in `creature.c` and `spells.c` use it, so
  a monster keeps its color during movement and spell animations.
* `monster_template_get_symbol` is unchanged.

Acceptance checks:
* L0: every template has a color other than black; the glitch template
  gets a defined color.
* L0: `monster_template_get_glyph` low byte equals
  `monster_template_get_symbol` for every index, and an out-of-bounds index
  behaves like the symbol accessor does today.
* L2, C headless: a visible monster in a lit cell returns its symbol with
  its color bits from `get_loc_symbol`; an unseen monster still shows the
  terrain or item under it.
* Changelog entry: monsters are colored.

Out of scope: monster recall text; the sidebar.

### MC5. Sidebar Monster Colors (fan-out)

Status: open. Depends on: MC4.
Size: S. Complexity: Medium. Agent: standard.

Owns: the monsters panel in [sidebar.rs](../../../src/user_interface/sidebar.rs)
and the attribute-capable draw path it needs in
[layout.rs](../../../src/user_interface/layout.rs). Must not touch: the
equipment and inventory panels, mouse hit testing.

Behavior:
* The visible-monsters panel draws each monster's symbol with the same
  attribute the map uses, then the name and count plain.
* Row text, ordering, grouping, truncation, and the blind and hallucination
  blanking stay as they are. Panel text assertions keep passing.
* Hover and click handling read the panel rows by position and do not
  change.

Acceptance checks:
* L0: the rendered lines are identical to today's, and a separate per-row
  attribute list reports the symbol cell with the template color.
* L0: blind and hallucinating panels carry no attributes.
* L1, file-backed `newterm()`: the symbol cell of a monster row reads back
  with the expected pair, and the rest of the row has none; check the panel at
  a split size and at the minimum size.
* Changelog entry: the visible-monsters panel uses the map colors.

Out of scope: coloring names; the equipment and inventory panels.

## Remaining Gaps

* **Remembered tiles.** Cells that are not lit show as blanks, so
  dimming remembered walls is a separate feature.
* **Lava and trees.** Neither exists in the game, so no role is defined.
* **Inventory and equipment.** These panels show names without glyphs and
  stay uncolored.
* **Color blindness and light terminals.** The palette assumes a dark
  background; no contrast setting is planned (MCD4).
* **Real-terminal check.** Pair attributes are tested with file-backed
  screens; how the colors look in a real 8-color and 256-color terminal is
  a manual check.
