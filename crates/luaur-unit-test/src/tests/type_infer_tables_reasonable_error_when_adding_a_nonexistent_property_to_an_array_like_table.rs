//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:2038:type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table`
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
//!   - type_ref -> record CannotExtendTable (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - type_ref -> record UnknownProperty (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table

#[cfg(test)]
#[test]
fn type_infer_tables_reasonable_error_when_adding_a_nonexistent_property_to_an_array_like_table() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::cannot_extend_table::CannotExtendTable;
    use luaur_analysis::records::unknown_property::UnknownProperty;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        function mkA() return {"value"} end
        local A = mkA()
        A.B = "Hello"
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let cet = type_error_data_ref::<CannotExtendTable>(&result.errors[0])
            .unwrap_or_else(|| panic!("Expected CannotExtendTable but got {:?}", result.errors[0]));
        assert_eq!("B", cet.prop());
    } else {
        let up = type_error_data_ref::<UnknownProperty>(&result.errors[0])
            .unwrap_or_else(|| panic!("Expected UnknownProperty but got {:?}", result.errors[0]));
        assert_eq!("B", up.key());
    }
}
