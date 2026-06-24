//! Regression test (NOT an upstream cxx:Test) for a Rust-port-specific deviation.
//!
//! C++ `Unifier::tryUnify` (the public entry) resets `iterationCount = 0` at the
//! start of every top-level unification (Unifier.cpp:385/1396). Our port's
//! `TypeChecker::unify` / `TypeChecker::try_unify` originally called the
//! recursive `tryUnify_` directly and skipped that reset, so the iteration
//! counter accumulated across EVERY old-solver unification in a module check and
//! spuriously tripped `LuauTypeInferIterationLimit` (this is what broke
//! `luau_subtyping_is_np_hard`).
//!
//! This test does several independent, moderately-expensive graph-coloring
//! subtype checks in ONE module. Each is well under the iteration limit on its
//! own, but their SUM far exceeds it — so if the per-unification reset ever
//! regresses, the later checks raise spurious `UnificationTooComplex` and this
//! test fails. It also exercises the reflexive structural-equality fast-path
//! (`unifier_reflexive_equal`) that makes each check cheap despite our
//! alias-union types not being pointer-shared the way C++ interns them.

#[cfg(test)]
#[test]
fn type_infer_functions_iteration_limit_resets_per_unification() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
--!strict
type Red = "red"
type Blue = "blue"
type Color = Red | Blue
type Coloring = (Color) -> (Color) -> (Color) -> boolean
type Uncolorable = (Color) -> (Color) -> (Color) -> false

type Line = Coloring
  & ((Red) -> (Red) -> (Color) -> false)
  & ((Blue) -> (Blue) -> (Color) -> false)
  & ((Color) -> (Red) -> (Red) -> false)
  & ((Color) -> (Blue) -> (Blue) -> false)

type Triangle = Line
  & ((Red) -> (Color) -> (Red) -> false)
  & ((Blue) -> (Color) -> (Blue) -> false)

local t : Triangle

-- Each of these assignments runs the full (expensive) Triangle <: Uncolorable
-- unification. Individually each is under LuauTypeInferIterationLimit; their sum
-- is many times over it. With the per-unification counter reset they all pass;
-- without it the cumulative count trips UnificationTooComplex on a later one.
local z1 : Uncolorable = t
local z2 : Uncolorable = t
local z3 : Uncolorable = t
local z4 : Uncolorable = t
local z5 : Uncolorable = t
local z6 : Uncolorable = t
local z7 : Uncolorable = t
local z8 : Uncolorable = t
    "#,
        ),
        None,
    );

    // Triangle is uncolorable, so every assignment is sound: zero errors, and in
    // particular zero spurious UnificationTooComplex from a leaked iteration count.
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
