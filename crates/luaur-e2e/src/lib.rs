//! `luaur-e2e` — end-to-end / integration test suite for the luaur project.
//!
//! This crate carries no production code: it exists purely as the home for the
//! integration tests under `tests/`, which drive the six shipping CLI binaries
//! (`luaur`, `luaur-analyze`, `luaur-ast`, `luaur-compile`, `luaur-bytecode`,
//! `luaur-reduce`) and the `luaur` umbrella library API as a real product —
//! exercising IO, feature flags, and hostile edge cases.
//!
//! The shared helpers below are used by several of the integration test files.

/// Names of the six CLI binaries under test, as `assert_cmd::cargo_bin` expects.
pub const BINARIES: [&str; 6] = [
    "luaur",
    "luaur-analyze",
    "luaur-ast",
    "luaur-compile",
    "luaur-bytecode",
    "luaur-reduce",
];
