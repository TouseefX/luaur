//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3761:type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible

#[cfg(test)]
#[test]
fn type_infer_tables_a_free_shape_cannot_turn_into_a_scalar_if_it_is_not_compatible() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(s): string
            local foo = s:absolutely_no_scalar_has_this_method()
            return s
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(3, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Parameter 's' has been reduced to never. This function is not callable with any possible value.",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Parameter 's' is required to be a subtype of '{ read absolutely_no_scalar_has_this_method: (never) -> (unknown, ...unknown) }' here.",
            to_string_type_error(&result.errors[1])
        );
        assert_eq!(
            "Parameter 's' is required to be a subtype of 'string' here.",
            to_string_type_error(&result.errors[2])
        );
        assert_eq!(
            "(never) -> string",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);

        let expected = "Expected this to be 'string', but got 't1 where t1 = {+ absolutely_no_scalar_has_this_method: (t1) -> (a, b...) +}'\ncaused by:\n  The given type's metatable does not satisfy the requirements.\nTable type 'typeof(string)' not compatible with type 't1 where t1 = {+ absolutely_no_scalar_has_this_method: (t1) -> (a, b...) +}' because the former is missing field 'absolutely_no_scalar_has_this_method'";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));

        assert_eq!(
            "<a, b...>(t1) -> string where t1 = {+ absolutely_no_scalar_has_this_method: (t1) -> (a, b...) +}",
            to_string_type_id(fixture.require_type_string(&String::from("f")))
        );
    }
}
