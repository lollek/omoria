---
applyTo: "src/**/*.h"
description: "Use when documenting C header functions, parameters, or struct fields, especially declarations mirrored across the Rust C ABI."
---

# C Header Documentation

- Before writing or updating header documentation, inspect nearby documented headers and match their Doxygen style.
- Use `/** ... */` comments with `@brief`; document every function parameter with `@param` and non-void return values with `@return`.
- Document public struct fields with concise member comments; do not describe fields as function parameters.
- When a C struct mirrors a Rust `#[repr(C)]` type, keep field names, types, and order synchronized. Use the same name for the same semantic value across both declarations and their consumers.