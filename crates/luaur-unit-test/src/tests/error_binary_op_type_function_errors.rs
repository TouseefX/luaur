//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:39:error_binary_op_type_function_errors`
//! Source: `tests/Error.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Error.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/Error.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item error_binary_op_type_function_errors

#[cfg(test)]
#[test]
fn error_binary_op_type_function_errors() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local x = 1 + "foo"
    "#,
        ),
        None,
    );

    assert_eq!(result.errors.len(), 1);

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Operator '+' could not be applied to operands of types number and string; there is no corresponding overload for __add",
            to_string_type_error(&result.errors[0])
        );
    } else {
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
    }
}
