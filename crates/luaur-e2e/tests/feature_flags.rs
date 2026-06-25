//! Feature-flag tests.
//!
//! (a) Luau FastFlags via the CLI `--fflags=` option and via the library flag
//!     API in `luaur-common` (`FFlag::<Name>.get()/.set()`), confirming a flag
//!     toggles observably and that `--fflags=` is accepted without error.
//! (b) The Cargo feature matrix is checked by `scripts/check-features.sh` (run
//!     separately, NOT as a recursive-cargo #[test]).

mod common;

use common::write_script;
use predicates::prelude::*;

// ---------------------------------------------------------------------------
// (a) FastFlags — library API
// ---------------------------------------------------------------------------

#[test]
fn library_fflag_toggle_is_observable() {
    use luaur_common::FFlag;
    // The flag API round-trips: setting a value is immediately observable via
    // get(). (FValue<bool> is a process-global, matching Luau's FFlag storage.)
    let original = FFlag::LuauCompileFoldOptimize.get();

    FFlag::LuauCompileFoldOptimize.set(false);
    assert!(
        !FFlag::LuauCompileFoldOptimize.get(),
        "flag should read back false"
    );

    FFlag::LuauCompileFoldOptimize.set(true);
    assert!(
        FFlag::LuauCompileFoldOptimize.get(),
        "flag should read back true"
    );

    // Restore so we don't perturb other tests sharing this process.
    FFlag::LuauCompileFoldOptimize.set(original);
}

#[test]
fn set_all_flags_round_trips() {
    // The CLI's setLuauFlagsDefault() analog must run without panicking and be
    // observable on at least one representative flag.
    luaur_common::set_all_flags(true);
    assert!(
        luaur_common::FFlag::LuauConst2.get(),
        "set_all_flags(true) should enable Luau flags"
    );
}

// ---------------------------------------------------------------------------
// (a) FastFlags — CLI acceptance
// ---------------------------------------------------------------------------

#[test]
fn cli_global_fflags_true_is_accepted() {
    let (_dir, path) = write_script("p.luau", "return 1 + 2\n");
    // The documented global form `--fflags=true` enables all flags and must be
    // accepted cleanly (exit 0), still producing the disassembly.
    common::bin("luaur-compile")
        .arg("--fflags=true")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("RETURN"));
}

#[test]
fn cli_global_fflags_false_is_accepted() {
    let (_dir, path) = write_script("p.luau", "print('ff-false-ok')\n");
    common::bin("luaur")
        .arg("--fflags=false")
        .arg(&path)
        .assert()
        .success()
        .stdout(predicate::str::contains("ff-false-ok"));
}

#[test]
fn cli_named_fflag_does_not_fail() {
    let (_dir, path) = write_script("p.luau", "return 1\n");
    // A named-flag toggle is accepted without a non-zero exit even when the
    // setter warns about an unrecognized name (faithful: setLuauFlags does not
    // abort on an unknown flag).
    common::bin("luaur-compile")
        .arg("--fflags=LuauCompileFoldOptimize=true")
        .arg(&path)
        .assert()
        .success();
}

#[test]
fn analyze_accepts_fflags_option() {
    let (_dir, path) = write_script("good.luau", "--!strict\nlocal x: number = 1\nreturn x\n");
    common::bin("luaur-analyze")
        .arg("--fflags=true")
        .arg(&path)
        .assert()
        .success();
}
