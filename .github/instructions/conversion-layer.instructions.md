---
applyTo: "src/conversion/**"
description: "Use when writing or editing C-to-Rust type conversion modules. Covers bidirectional mapping patterns and handling invalid C values."
---

# Conversion Layer Conventions

The `src/conversion/` module provides bidirectional mappings between C integer codes and Rust enum types.

## Pattern

Each conversion module provides two functions:

```rust
use crate::model;

pub fn from_usize(value: usize) -> Option<model::MyType> {
    match value {
        0 => Some(model::MyType::VariantA),
        1 => Some(model::MyType::VariantB),
        _ => None,  // Unknown C value
    }
}

pub fn to_usize(value: model::MyType) -> usize {
    match value {
        model::MyType::VariantA => 0,
        model::MyType::VariantB => 1,
    }
}
```

## Rules

- Return `Option` from `from_*` functions — C code may pass invalid values.
- The `to_*` direction is infallible (every Rust variant has a C value).
- Keep match arms exhaustive — no wildcard `_` in `to_*` functions (compiler catches missing variants).
- Use `usize` as the C-side type unless the domain uses something else (e.g., `i64` for signed values).
- Import types from `crate::model`, not from `crate::data`.

## Naming

- Module name matches the type: `class.rs` for `Class`, `currency.rs` for `Currency`.
- Functions: `from_usize`, `to_usize` (or `from_i64`/`to_i64` for signed types).
- For richer conversions: `from_usize_or_blank(class, slot)` when context is needed.
