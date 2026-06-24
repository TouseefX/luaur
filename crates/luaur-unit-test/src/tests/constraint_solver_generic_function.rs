//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ConstraintSolver.test.cpp:20:constraint_solver_generic_function`
//! Source: `tests/ConstraintSolver.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ConstraintSolver.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/ConstraintSolver.test.cpp
//! - outgoing:
//!   - translates_to -> rust_item constraint_solver_generic_function

#[cfg(test)]
#[test]
fn constraint_solver_generic_function() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);

    fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function id(a)
            return a
        end
    "#,
        ),
        None,
    );

    assert_eq!(
        "<a>(a) -> a",
        to_string_type_id(fixture.require_type_string(&String::from("id")))
    );
}
