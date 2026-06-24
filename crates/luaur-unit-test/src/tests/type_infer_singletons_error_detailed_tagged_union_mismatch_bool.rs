//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.singletons.test.cpp:435:type_infer_singletons_error_detailed_tagged_union_mismatch_bool`
//! Source: `tests/TypeInfer.singletons.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.singletons.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.singletons.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method Position::missing (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item type_infer_singletons_error_detailed_tagged_union_mismatch_bool

#[cfg(test)]
#[test]
fn type_infer_singletons_error_detailed_tagged_union_mismatch_bool() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Good = { success: true, result: string }
type Bad = { success: false, error: string }
type Result = Good | Bad

local a: Result = { success = false, result = 'something' }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Table type '{ result: string, success: false }' not compatible with type 'Bad' because the former is missing field 'error'",
            to_string_type_error(&result.errors[0])
        );
    } else {
        let expected = "Expected this to be 'Bad | Good', but got 'a'
caused by:
  None of the union options are compatible. For example:
Table type 'a' not compatible with type 'Bad' because the former is missing field 'error'";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    }
}
