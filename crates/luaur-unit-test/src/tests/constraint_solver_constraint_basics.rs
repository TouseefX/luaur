//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:10:constraint_solver_constraint_basics`
//! Source: `tests/ConstraintSolver.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ConstraintSolver.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/ConstraintSolver.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item constraint_solver_constraint_basics

#[cfg(test)]
#[test]
fn constraint_solver_constraint_basics() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = 55
        local b = a
    "#,
        ),
        None,
    );

    assert_eq!(
        "number",
        to_string_type_id(fixture.require_type_string(&String::from("b")))
    );
}
