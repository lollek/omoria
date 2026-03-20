---
name: c-to-rust-migration
description: "Migrate C code to Rust in omoria. Use when porting a C module, function, or subsystem to Rust. Covers analysis, module placement, FFI interop, test strategy, and the full RED-GREEN-REFACTOR workflow with migration plan references."
argument-hint: "Which C module or function to migrate"
---

# C-to-Rust Migration

Guided workflow for porting C code to idiomatic Rust in the omoria roguelike, following the project's TDD contract.

## When to use

- Porting a C function, file, or subsystem to Rust
- Planning the migration of a new module
- Understanding how existing ports were structured

## Reference documents

- [Migration plan](./references/migration-plan-summary.md) — phases, priorities, and what's already done
- [Completed port example](./references/trap-port-example.md) — how `traps.c` was ported

## Procedure

### Step 1: Scope and analyze

1. Read the C source file to understand:
   - What functions exist and what they do
   - Dependencies (includes, globals, other C functions called)
   - Which functions are pure vs. side-effectful
2. Check the [migration plan](./references/migration-plan-summary.md) for context on priority and phase.
3. Propose a **breakdown** if the file has multiple concerns (data, logic, interop). Use the traps example as a model:
   - `data.rs` — static definitions / templates
   - `logic.rs` or domain-named file — pure business logic
   - `interop.rs` — `extern "C"` wrappers for C callers
   - `mod.rs` — public API and re-exports

### Step 2: Choose target location

Follow existing module organization (domain-vertical slicing):

| Domain | Location |
|--------|----------|
| Dungeon/map | `src/dungeon/` |
| Combat | `src/combat/` |
| Player | `src/player/` |
| Items | `src/inventory/`, `src/equipment.rs`, `src/generate_item/` |
| Monsters | `src/generate_monster/` |
| Magic | `src/magic/` |
| Casino | `src/casino/` (currently C only) |
| Player actions | `src/player_action/` |
| Flow/effects | `src/flow/` |
| Data/types | `src/model/`, `src/data/` |
| Conversions | `src/conversion/` |

### Step 3: RED — write failing tests

1. Identify the **key behaviors** to test (happy path, edge cases, legacy quirks).
2. Write `#[cfg(test)]` tests that describe each behavior.
3. If the function uses RNG, test the `_with_rng` variant with seeded `StdRng`.
4. Add stubs so it compiles but fails.
5. **Stop — ask for navigator review.**

### Step 4: GREEN — minimal implementation

1. Port the C logic to Rust, matching behavior exactly (unless explicitly fixing a bug).
2. Use idiomatic Rust (enums, `Option`, pattern matching) but preserve semantics.
3. If C code calls this function, add an `extern "C"` wrapper in `interop.rs`.
4. Make all tests pass.
5. **Stop — ask for navigator review.**

### Step 5: REFACTOR — cleanup

1. Improve naming, add doc comments for non-obvious logic.
2. Extract shared patterns (e.g., template structs for static data).
3. Reduce duplication between similar functions.
4. All tests must stay green.
5. **Stop — ask for navigator review.**

### Step 6: Finalize

1. `cargo test --lib` — all tests pass.
2. No new compiler warnings.
3. Update `CHANGELOG.md` under `## Unreleased`.
4. If the C function is now fully replaced, note which C code can be removed (but don't delete it without navigator approval).

## Key patterns to follow

### RNG injection
```rust
// Public wrapper
pub fn my_function(args...) -> Result {
    my_function_with_rng(&mut rand::thread_rng(), args...)
}

// Testable core
pub fn my_function_with_rng(rng: &mut impl Rng, args...) -> Result {
    let roll = randint_with_rng(rng, max);
    // ...
}
```

### FFI interop
```rust
// In interop.rs — thin shim only
#[no_mangle]
pub extern "C" fn c_name(arg: libc::c_long) -> libc::c_long {
    domain_module::rust_function(arg)
}
```

### Static data migration
```rust
// Replace C arrays with Rust const slices
pub const MY_DATA: &[MyTemplate] = &[
    MyTemplate { name: "foo", level: 1, ... },
    MyTemplate { name: "bar", level: 2, ... },
];
```
