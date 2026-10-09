# Trap Partial-Port Example

The `traps.c` migration demonstrates a **partial port**: data and placement are
Rust, while activation remains C. See the authoritative
[migration plan](../../../../docs/c-to-rust-migration-plan.md) and
[trap details](../../../../docs/migration/traps-migration.md).

## C source: `traps.c`

Mixed concerns: static data, placement logic, effect handlers, and town entrance dispatch.

## Current Rust modules: `src/dungeon/trap/`

```
src/dungeon/trap/
├── mod.rs           # Public API, re-exports, module docs
├── data.rs          # Static trap templates (TrapTemplate struct + TRAP_LIST const)
├── data_tests.rs    # Data validation tests
├── placement.rs     # place_trap, change_trap logic (TrapList enum, tval_for, apply_template_to_item)
├── globals.rs       # Functions that mutate global dungeon state (unsafe wrappers)
├── interop.rs       # extern "C" shims: place_trap, change_trap, place_rubble
└── test_support.rs  # Shared test helpers
```

## Key decisions

### Unified data list
C had two nearly-identical arrays (`trap_lista`, `trap_listb`) differing only by `tval`. Rust stores one `TRAP_LIST` and sets `tval` at placement time based on `TrapList::A` vs `TrapList::B`.

### Template struct
Only varying fields are in `TrapTemplate`. Fields such as flags and weight are
cleared in `apply_template_to_item`; `tval` is selected during placement. Open
pits are always visible and subval 19 always uses the closed-door tval.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrapTemplate {
    pub name: &'static str,
    pub level: i64,
    pub subval: i64,
    pub damage: &'static str,
    pub cost: i64,
}
```

### Interop layer
Thin C ABI wrappers in [interop.rs](../../../../src/dungeon/trap/interop.rs)
export `place_trap`, `change_trap`, and `place_rubble`, using `libc::c_long`.
The current wrapper below is not a checked interface: it casts inputs to `usize`
and relies on valid coordinates and template indices.

```rust
use crate::dungeon::trap::{place_trap_global, TrapList};

#[no_mangle]
pub extern "C" fn place_trap(y: libc::c_long, x: libc::c_long, typ: libc::c_long, subval: libc::c_long) {
    let list = if typ == 1 { TrapList::A } else { TrapList::B };
    unsafe { place_trap_global(y as usize, x as usize, list, subval as usize); }
}
```

### Indexing convention
Placement uses 1-based subvals (1 through 20). Rust `template_for` translates
with `let index = subval - 1;`; the open pit is `TRAP_LIST[0]`, not `[1]`.
Activation values such as 99 and 101-123 are not template indices.

## What's NOT yet ported

- Trap effect handlers (`ht__*` functions)
- `hit_trap` dispatcher
- `trigger_trap` (chest traps)
- Town/special-tile dispatch (101-122; 119 has no explicit case)
- Whirlpool activation (123)

The remaining C ABI is `hit_trap(const long *y, const long *x)` and
`trigger_trap(long y, long x)`. A future Rust export must preserve the pointer
arguments for `hit_trap`; `trigger_trap` reads chest flags from the item, not an
extra flags parameter. These are future work, not existing Rust exports.
