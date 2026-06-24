//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3674:type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type`
//! Source: `tests/TypeInfer.tables.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tables.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tables.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type

#[cfg(test)]
#[test]
fn type_infer_tables_scalar_is_not_a_subtype_of_a_compatible_polymorphic_shape_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(s)
            return s:absolutely_no_scalar_has_this_method()
        end

        f("foo" :: string)
        f("bar" :: "bar")
        f("baz" :: "bar" | "baz")
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(4, result.errors.len(), "{:?}", result.errors);

        for error in &result.errors {
            let tm = type_error_data_ref::<TypeMismatch>(error).expect("expected TypeMismatch");
            assert_eq!("typeof(string)", to_string_type_id(tm.given_type));
            assert_eq!(
                "t1 where t1 = { read absolutely_no_scalar_has_this_method: (t1) -> (a...) }",
                to_string_type_id(tm.wanted_type)
            );
        }
    } else {
        assert_eq!(3, result.errors.len(), "{:?}", result.errors);

        let expected1 = "Expected this to be 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}', but got 'string'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
        assert_eq!(expected1, to_string_type_error(&result.errors[0]));

        let expected2 = "Expected this to be 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}', but got '\"bar\"'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
        assert_eq!(expected2, to_string_type_error(&result.errors[1]));

        let expected3 = "Expected this to be\n\t't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}'\nbut got\n\t'\"bar\" | \"baz\"'\ncaused by:\n  Not all union options are compatible.\nExpected this to be 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}', but got '\"bar\"'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {- absolutely_no_scalar_has_this_method: (t1) -> (a...) -}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
        assert_eq!(expected3, to_string_type_error(&result.errors[2]));
    }
}
