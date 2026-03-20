---
applyTo: "**/*_extern.rs,**/interop.rs"
description: "Use when writing or editing FFI extern wrappers, C ABI shims, or interop modules that bridge Rust logic to C callers."
---

# FFI Extern File Conventions

These files are **thin shims** — they translate C types into Rust types and delegate to the real implementation. Never put business logic here.

## Pattern

```rust
use libc;
use crate::some_module;

#[no_mangle]
pub extern "C" fn c_function_name(arg: libc::c_long) -> libc::c_long {
    some_module::rust_function(arg)
}
```

## Rules

- Every exported function needs `#[no_mangle]` and `pub extern "C"`.
- Use `libc::` types for all parameters and return types (`c_long`, `c_char`, etc.).
- Keep the body to a single delegation call (type conversions + call into domain module).
- If a function needs string returns, use `CString` with `lazy_static!` for static strings:
  ```rust
  lazy_static! {
      static ref MY_CSTR: CString = CString::new(my_module::MY_STRING).unwrap();
  }
  #[no_mangle]
  pub extern "C" fn get_my_string() -> *const libc::c_char {
      MY_CSTR.as_ptr()
  }
  ```
- Domain-specific interop goes in the domain module (e.g., `dungeon/trap/interop.rs`), not in top-level `*_extern.rs`.
- Top-level `*_extern.rs` files are for broadly shared utilities (RNG, constants, terminal).

## Naming

- If the C function name matches the Rust function, use the same name: `fn randint(...)`.

## Safety

- Minimize `unsafe` blocks. Prefer safe Rust in the domain module.
- When `unsafe` is required (e.g., global state mutation), keep it in the interop layer and document why.
