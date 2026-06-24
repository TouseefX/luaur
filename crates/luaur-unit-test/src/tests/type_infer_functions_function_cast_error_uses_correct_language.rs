//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.functions.test.cpp:1152:type_infer_functions_function_cast_error_uses_correct_language`
//! Source: `tests/TypeInfer.functions.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.functions.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.functions.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_functions_function_cast_error_uses_correct_language

#[cfg(test)]
#[test]
fn type_infer_functions_function_cast_error_uses_correct_language() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(a, b): number
            return 0
        end

        local a: (string)->number = foo
        local b: (number, number)->(number, number) = foo

        local c: (string, number)->number = foo -- no error
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    let tm1 = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("(string) -> number", to_string_type_id(tm1.wanted_type));
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(unknown, unknown) -> number",
            to_string_type_id(tm1.given_type)
        );
    } else {
        assert_eq!(
            "(string, *error-type*) -> number",
            to_string_type_id(tm1.given_type)
        );
    }

    let tm2 = unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!(
        "(number, number) -> (number, number)",
        to_string_type_id(tm2.wanted_type)
    );
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "(unknown, unknown) -> number",
            to_string_type_id(tm1.given_type)
        );
    } else {
        assert_eq!(
            "(string, *error-type*) -> number",
            to_string_type_id(tm2.given_type)
        );
    }
}
