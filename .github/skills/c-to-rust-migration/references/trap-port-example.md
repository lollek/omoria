# Trap Port Example

The `traps.c` migration is the best example of a completed C-to-Rust port in this repo. It demonstrates how to split a C file with multiple concerns into focused Rust modules.

## C source: `traps.c` (590 lines)

Mixed concerns: static data, placement logic, effect handlers, and town entrance dispatch.

## Rust target: `src/dungeon/trap/`

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
Only fields that actually vary are in `TrapTemplate`. Constant fields (flags, weight, etc.) are set to zero in `apply_template_to_item`.

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
Thin C ABI wrappers in `interop.rs` let C code call into Rust without changes:

```rust
#[no_mangle]
pub extern "C" fn place_trap(y: libc::c_long, x: libc::c_long, typ: libc::c_long, subval: libc::c_long) {
    let list = if typ == 1 { TrapList::A } else { TrapList::B };
    unsafe { place_trap_global(y as usize, x as usize, list, subval as usize); }
}
```

### Indexing convention
C uses 1-based subvals. The Rust `template_for` function handles the translation: `let index = subval - 1;`.

## What's NOT yet ported

- Trap effect handlers (`ht__*` functions)
- `hit_trap` dispatcher
- `trigger_trap` (chest traps)
- Town entrance logic (cases 101-123)

These are tracked in the migration plan as future work.
