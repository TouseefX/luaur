//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:152:type_infer_tables_index_expression_is_checked_against_the_indexer_type`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotExtendTable (Analysis/include/Luau/Error.h)
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_index_expression_is_checked_against_the_indexer_type

#[cfg(test)]
#[test]
fn type_infer_tables_index_expression_is_checked_against_the_indexer_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::cannot_extend_table::CannotExtendTable;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(t: {[boolean]: number})
            t["hello"] = 15
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        type_error_data_ref::<CannotExtendTable>(&result.errors[0])
            .expect("expected CannotExtendTable");
    } else {
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    }
}
