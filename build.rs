fn main() {
    // Build scripting note:
    // Cargo does *not* expose the crate-level `cfg(test)` to build.rs via env vars.
    // So we can't reliably detect `cargo test` here.
    //
    // Instead, we compile the shim only when the caller explicitly opts in by
    // setting `OMORIA_BUILD_TEST_SHIMS=1` (done by `cargo test` via .cargo/config.toml).
    if std::env::var_os("OMORIA_BUILD_TEST_SHIMS").is_some() {
        cc::Build::new()
            .file("mock/msg_print.c")
            .compile("msg_print");
    }
}
