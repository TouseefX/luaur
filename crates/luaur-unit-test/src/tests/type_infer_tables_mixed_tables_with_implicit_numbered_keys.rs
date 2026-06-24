//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3571:type_infer_tables_mixed_tables_with_implicit_numbered_keys`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_tables_mixed_tables_with_implicit_numbered_keys

#[cfg(test)]
#[test]
fn type_infer_tables_mixed_tables_with_implicit_numbered_keys() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t: { [string]: number } = { 5, 6, 7 }
    "#,
        ),
        None,
    );

    assert_eq!(3, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        for error in &result.errors {
            assert_eq!(
                "Unexpected array-like table item: the indexer key type of this table is not `number`.",
                to_string_type_error(error)
            );
        }
    } else {
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[1])
        );
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[2])
        );
    }
}
