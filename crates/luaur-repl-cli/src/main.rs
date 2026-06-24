//! `luaur` — the Luau REPL / script runner (binary entry point).
//!
//! Thin wrapper over the library `main()`, which marshals argv and dispatches to
//! `repl_main` (faithful port of the upstream `luau` CLI in CLI/src/ReplEntry.cpp).

fn main() {
    luaur_repl_cli::functions::main::main();
}
