//! Shared helpers for the luaur end-to-end integration tests.
//!
//! Included via `mod common;` from each `tests/*.rs` file. Not all helpers are
//! used by every test file, so individual unused items are tolerated.
#![allow(dead_code)]

use assert_cmd::Command;
use std::io::Write;
use std::path::PathBuf;
use tempfile::TempDir;

/// Locate one of the six shipping CLI binaries and wrap it in an
/// `assert_cmd::Command`.
///
/// `assert_cmd::Command::cargo_bin` only works for binaries belonging to the
/// *current* crate (it reads `CARGO_BIN_EXE_<name>`, which cargo sets only for
/// the package under test). Our binaries live in sibling workspace crates, so
/// we resolve their path from `CARGO_MANIFEST_DIR` (the `luaur-e2e` crate dir)
/// up to the workspace root, then into `target/<profile>/<name>`. This is also
/// robust to the project's `build-dir` relocation (final binaries stay under
/// `./target`, only intermediates move).
pub fn bin(name: &str) -> Command {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // crates/luaur-e2e -> crates -> <workspace root>
    let workspace_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root above crates/luaur-e2e")
        .to_path_buf();
    let target = workspace_root.join("target");

    // Tests usually build in the dev (`debug`) profile; honor a release run too.
    let candidates = [
        target.join("debug").join(name),
        target.join("release").join(name),
    ];
    let path = candidates.iter().find(|p| p.exists()).unwrap_or_else(|| {
        panic!(
            "could not locate binary {name}; looked in {:?}. Build the workspace bins first.",
            candidates
        )
    });
    Command::new(path)
}

/// Create a fresh temp dir and write `source` into `<dir>/<name>`, returning the
/// dir (which must be kept alive for the file to persist) and the full path.
pub fn write_script(name: &str, source: &str) -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let path = dir.path().join(name);
    let mut f = std::fs::File::create(&path).expect("create script file");
    f.write_all(source.as_bytes()).expect("write script");
    f.flush().expect("flush script");
    (dir, path)
}

/// A path inside a brand-new temp dir that does not exist on disk.
pub fn missing_path() -> (TempDir, PathBuf) {
    let dir = tempfile::tempdir().expect("create tempdir");
    let path = dir.path().join("does-not-exist.luau");
    (dir, path)
}
