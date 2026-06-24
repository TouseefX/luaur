//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:1792:type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors`
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
//!   - type_ref -> record MissingProperties (Analysis/include/Luau/Error.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors

#[cfg(test)]
#[test]
fn type_infer_tables_table_subtyping_with_missing_props_dont_report_multiple_errors() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::missing_properties::{Context, MissingProperties};
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(vec1: {x: number}): {x: number, y: number, z: number}
            return vec1
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        let expected = "Expected this to be\n\t'{ x: number, y: number, z: number }'\nbut got\n\t'{ x: number }'";
        assert_eq!(expected, to_string_type_error(&result.errors[0]));
    } else {
        let mp = type_error_data_ref::<MissingProperties>(&result.errors[0])
            .unwrap_or_else(|| panic!("Expected MissingProperties but got {:?}", result.errors[0]));
        assert_eq!(Context::Missing, mp.context());
        assert_eq!(2, mp.properties().len());
        assert_eq!("y", mp.properties()[0].as_str());
        assert_eq!("z", mp.properties()[1].as_str());

        assert_eq!(
            "{ x: number, y: number, z: number }",
            to_string_type_id(mp.superType())
        );
        assert_eq!("{ x: number }", to_string_type_id(mp.subType()));
    }
}
