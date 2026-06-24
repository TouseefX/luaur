//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:140:type_function_unsolvable_function`
//! Source: `tests/TypeFunction.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeFunction.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file Analysis/include/Luau/ConstraintSolver.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeFunction.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - translates_to -> rust_item type_function_unsolvable_function

#[cfg(test)]
#[test]
fn type_function_unsolvable_function() {
    use crate::records::type_function_fixture::TypeFunctionFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = TypeFunctionFixture::type_function_fixture();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local impossible: <T>(Swap<T>) -> Swap<Swap<T>>
        local a = impossible(123)
        local b = impossible(true)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "Expected this to be unreachable, but got 'number'",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Expected this to be unreachable, but got 'boolean'",
        to_string_type_error(&result.errors[1])
    );
}
