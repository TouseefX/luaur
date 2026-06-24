//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Error.test.cpp:59:error_unary_op_type_function_errors`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item error_unary_op_type_function_errors

#[cfg(test)]
#[test]
fn error_unary_op_type_function_errors() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend().options.retain_full_type_graphs = false;

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        local x = -"foo"
    "#,
        ),
        None,
    );

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(result.errors.len(), 2);
        assert_eq!(
            "Operator '-' could not be applied to operand of type string; there is no corresponding overload for __unm",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[1])
        );
    } else {
        assert_eq!(result.errors.len(), 1);
        assert_eq!(
            "Expected this to be 'number', but got 'string'",
            to_string_type_error(&result.errors[0])
        );
    }
}
