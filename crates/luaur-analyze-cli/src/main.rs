//! `luaur-analyze` — standalone Luau type-checker CLI (binary entry point).
//!
//! Thin wrapper over the library `main()` (faithful port of the upstream
//! `luau-analyze` CLI in CLI/src/Analyze.cpp).

fn main() {
    luaur_analyze_cli::functions::main::main();
}
