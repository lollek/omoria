# Issue #50: Item Name Refactoring

## Goal

Extract item name generation from `generic_item()` into dedicated, testable functions per item category.

This makes the code:
- More maintainable (each category in its own file)
- More testable (focused tests per category)
- More flexible (category-specific logic without bloating a monolithic function)
- **More correct** (current `generic_item()` has bugs; reference C `objdes()` for correct behavior)

## Current State

The `generic_item()` function in `src/data/item_name/subtype.rs` handles many unrelated item types.
We want to migrate each category to its own file under `src/data/item_name/subtype/`, following the pattern established by `melee_weapon.rs`, `armor.rs`, etc.

## Reference: C `objdes()` Function

The authoritative item naming logic is in `src/text_lines.c:objdes()`. It processes template names with these placeholders:

| Placeholder | Meaning | Example |
|-------------|---------|---------|
| `&` | Article/count position ("a", "an", or number) | `& sword` → `a sword` / `2 swords` |
| `~` | Plural suffix | `sword~` → `sword` / `swords` |
| `^` | Identification boundary (text after `^` only shown when identified) | `Sword^ (+1,+2)` |
| `\|` | Alternative text (removed by objdes) | |
| `%P0` | Damage string | `(2d6)` |
| `%P1` | p1 value (signed) | `+3` |
| `%P2` | tohit (signed) | `+1` |
| `%P3` | todam (signed) | `+2` |
| `%P4` | toac (signed) | `+1` |
| `%P5` | p1 value (unsigned) | `3` |
| `%P6` | ac value (unsigned) | `5` |

### Rust Naming Conventions (differences from C)

The new Rust item name functions should:
- Use **lowercase** (e.g., `small leather shield`, not `Small Leather Shield`)
- **Omit** the article prefix ("a"/"an") - just return the item name
- Include count when `number > 1` (e.g., `2 small leather shields`)
- Include "no more" when `number == 0`

## Item Types Currently Using `generic_item()`

From `src/data/item_name.rs`:

```rust
ItemType::Chime
| ItemType::ClosedDoor
| ItemType::DownStaircase
| ItemType::DownSteepStaircase
| ItemType::EntranceToStore
| ItemType::FlaskOfOil
| ItemType::GemHelm
| ItemType::Horn
| ItemType::Instrument
| ItemType::OpenDoor
| ItemType::Potion1
| ItemType::Potion2
| ItemType::Ring
| ItemType::Rod
| ItemType::Rubble
| ItemType::Scroll1
| ItemType::Scroll2
| ItemType::SecretDoor
| ItemType::SeenTrap
| ItemType::Shield
| ItemType::Staff
| ItemType::UnseenTrap
| ItemType::UpStaircase
| ItemType::UpSteepStaircase
| ItemType::Whirlpool
```

## Migration Plan

Each task = one PR. Work in TDD style (RED → GREEN → REFACTOR).

### Groupings

| New Module | Item Types | Complexity | Notes |
|------------|------------|------------|-------|
| `shield.rs` | Shield | Low | Simple subtype → name mapping + armor bonus |
| `flask.rs` | FlaskOfOil | Very Low | Just "flask(s) of oil" |
| `potion.rs` | Potion1, ~~Potion2~~ | Medium | Material prefix + identification. Potion2 is removed (empty enum). |
| `scroll.rs` | Scroll1, ~~Scroll2~~ | Medium | Identification effects. Scroll2 is removed (empty enum). |
| `ring.rs` | Ring | Medium | Subtype-known vs item-identified behavior; shows [+toac] only when item is identified |
| `staff.rs` | Staff | Medium | Uses `%W` prefix + charges + identification |
| `chime.rs` | Chime | Medium | Uses `%M` prefix + identification |
| `horn.rs` | Horn | Medium | Uses `%H` prefix + identification |
| `instrument.rs` | Instrument | Low | Fixed names per subtype |
| `gem_helm.rs` | GemHelm | Low | Material + "of gems" when identified |
| `rod.rs` | Rod | Unknown | Need to check if subtypes exist |
| `dungeon_feature.rs` | ClosedDoor, OpenDoor, SecretDoor, UpStaircase, DownStaircase, UpSteepStaircase, DownSteepStaircase, EntranceToStore, Rubble, Whirlpool, SeenTrap, UnseenTrap | Low | Dungeon features, rarely displayed as "items" |

### Task Checklist

#### Bug Fixes (existing code)

- [ ] **melee_weapon.rs**: Fix `executioners sword` → `executioner's sword` (missing apostrophe)
- [ ] **melee_weapon.rs**: Fix `nodachi` → `no-dachi` (missing hyphen)
- [ ] **melee_weapon.rs**: Fix `great flail` → `two-handed great flail` (missing prefix)
- [ ] **melee_weapon.rs**: Fix `cat-o-nine tails` → `cat-o'-nine-tails` (match template style)
- [ ] **armor.rs**: Fix `soft studded leather` → `soft studded armor` (wrong name)
- [ ] **armor.rs**: Add `maybe_number_of()` for quantity handling

#### New Item Types

- [x] `flask.rs` - FlaskOfOil
- [x] `shield.rs` - Shield
- [x] `instrument.rs` - Instrument
- [x] `gem_helm.rs` - GemHelm
- [x] `ring.rs` - Ring
- [ ] `potion.rs` - Potion1 (Potion2 removed)
- [ ] `scroll.rs` - Scroll1 (Scroll2 removed)
- [ ] `chime.rs` - Chime
- [ ] `horn.rs` - Horn
- [ ] `staff.rs` - Staff
- [ ] `rod.rs` - Rod
- [ ] `dungeon_feature.rs` - All dungeon features (doors, stairs, traps, etc.)

#### Cleanup

- [ ] Remove empty `generic_item()` and unused code from `subtype.rs`

### Suggested Order

1. **`flask.rs`** - Trivial, good warm-up
2. **`shield.rs`** - Simple, establishes pattern for armor-like items
3. **`instrument.rs`** - Simple fixed names
4. **`gem_helm.rs`** - Simple with identification
5. **`ring.rs`** - Medium, subtype-known vs item-identified behavior (no more %R placeholder)
6. **`potion.rs`** - Medium, introduces material prefix pattern
7. **`scroll.rs`** - Medium, similar to potion
8. **`chime.rs`** - Medium, uses `%M` prefix
9. **`horn.rs`** - Medium, uses `%H` prefix
10. **`staff.rs`** - Medium, has charges
11. **`rod.rs`** - Check what logic exists
12. **`dungeon_feature.rs`** - Group all terrain/features last
13. **Cleanup** - Remove `generic_item()` after all migrations

## Implementation Pattern

Follow `melee_weapon.rs` as the reference:

1. Create new file: `src/data/item_name/subtype/{category}.rs`
2. Add `pub mod {category};` to `src/data/item_name/subtype.rs`
3. Add `use` statement to `src/data/item_name.rs`
4. Update match arm in `generate()` to call new function
5. Remove old logic from `subtype_name()` in `subtype.rs`
6. Add tests covering identified/unidentified states

## Convention: Avoid "magic" subtype numbers

When a naming module depends on `item.subval`, prefer using a typed subtype enum rather than matching on raw integers.

Pattern:

1. Define/extend the subtype enum in `src/model/item_subtype.rs`.
   - Example: `GemHelmSubType::{IronHelm, SteelHelm}` and `HelmSubType::{ClothHat, ...}`.

2. Add a conversion module under `src/conversion/item_subtype/`.
   - Example: `src/conversion/item_subtype/helm.rs` maps `12..=23` to `HelmSubType`.
   - Example: `src/conversion/item_subtype/gem_helm.rs` maps `9/10` to `GemHelmSubType`.

3. In the naming module, decode using `conversion::item_subtype::from_i64(item.item_type(), item.subval)` and match on `ItemSubType::{...}`.

4. In tests, avoid hardcoded subtype numbers by computing `subval` from the typed subtype:

   - `conversion::item_subtype::to_usize(&ItemSubType::GemHelm(GemHelmSubType::IronHelm)) as i64`

This keeps the numeric mapping in one place and makes item-name logic easier to read and refactor.

## Notes

- `%R`, `%M`, `%H`, `%W` are placeholder patterns that get replaced elsewhere (likely material names for unidentified items)
- `Potion2` and `Scroll2` have been removed from the game (empty enums that panic)
- Some items have special display logic (charges, armor bonus, damage) - check `generic_item()` for what applies

## Related Files

- `src/data/item_name.rs` - Main dispatch function
- `src/data/item_name/subtype.rs` - Current `generic_item()` and `subtype_name()`
- `src/data/item_name/subtype/*.rs` - Individual category modules
- `src/data/item_name/helpers.rs` - Shared helper functions