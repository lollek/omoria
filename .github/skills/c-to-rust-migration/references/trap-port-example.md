# Trap Port Example

The `traps.c` migration is a **complete port** done in stages: data and
placement first, then a Rust dispatcher (TR1) with per-group handler files
that later stories (TR2-TR6) filled in, and finally deletion of the emptied C
file (TR7). See the authoritative
[epic backlog](../../../../docs/epics/backlog.md) and
[trap activation epic](../../../../docs/epics/completed/TR-trap-activation.md).

## C source: `traps.c` (deleted)

Mixed concerns: static data, placement logic, effect handlers, and town entrance dispatch.
[traps.h](../../../../src/traps.h) remains so C callers keep linking.

## Rust modules: `src/dungeon/trap/`

```
src/dungeon/trap/
├── mod.rs           # Public API, re-exports, module docs
├── data.rs          # Static trap templates (TrapTemplate struct + TRAP_LIST const)
├── data_tests.rs    # Data validation tests
├── placement.rs     # place_trap, change_trap logic (TrapList enum, tval_for, apply_template_to_item)
├── globals.rs       # Functions that mutate global dungeon state (unsafe wrappers)
├── interop.rs       # extern "C" shims: hit_trap, trigger_trap, place_trap, change_trap, place_rubble
├── activation.rs    # hit_trap dispatch, including town entrance routing
├── activation/
│   └── globals.rs   # Checked pointer adapter and C callback boundary
├── basic.rs         # Open pit and arrow effects with an injectable context
├── status.rs        # Sleep, blind, and confuse gas; slow, strength, and constitution darts
├── element.rs       # Fire, acid, corrosion-gas, and poison-gas traps
├── movement.rs      # Covered pit, trap door, teleport, chute, and whirlpool effects
├── spawn.rs         # Hidden object, rockfall, and summon monster effects
├── chest.rs         # Chest traps (trigger_trap) decoded from item flags
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
export `hit_trap`, `trigger_trap`, `place_trap`, `change_trap`, and
`place_rubble`, using `libc::c_long`. The placement wrapper below is not a
checked interface: it casts inputs to `usize` and relies on valid coordinates
and template indices.

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

### Pointer ABI

Rust exports `hit_trap(const long *y, const long *x)` with null checks and
bounded cell access, preserving its pointer ABI. Non-null pointers must still
be aligned and readable; calls require exclusive access to initialized game
globals. `trigger_trap(long y, long x)` reads chest flags from the item, not
an extra flags parameter, and decodes them into a typed `ChestTraps` snapshot.

## What stays in C

Handlers call retained C through FFI, owned by later epics: `take_hit` and
`damroll`, `lose_stat`, item placement (`place_random_dungeon_item`, `pusht`,
`delete_object`), monster summons, and store, bank, casino, fortress, and
house entry. Town tile routing is Rust; subval 119 still falls through to the
unknown-trap message.
