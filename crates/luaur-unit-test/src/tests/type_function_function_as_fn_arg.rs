//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeFunction.test.cpp:109:type_function_function_as_fn_arg`
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
//!   - translates_to -> rust_item type_function_function_as_fn_arg

#[cfg(test)]
#[test]
fn type_function_function_as_fn_arg() {
    use crate::records::type_function_fixture::TypeFunctionFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = TypeFunctionFixture::type_function_fixture();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local swapper: <T>(Swap<T>) -> T
        local a = swapper(123)
        local b = swapper(false)
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "unknown",
        to_string_type_id(fixture.base.require_type_string(&String::from("a")))
    );
    assert_eq!(
        "unknown",
        to_string_type_id(fixture.base.require_type_string(&String::from("b")))
    );
    assert_eq!(
        "Expected this to be unreachable, but got 'number'",
        to_string_type_error(&result.errors[0])
    );
    assert_eq!(
        "Expected this to be unreachable, but got 'boolean'",
        to_string_type_error(&result.errors[1])
    );
}
