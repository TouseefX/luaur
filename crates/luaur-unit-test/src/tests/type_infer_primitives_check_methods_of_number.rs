//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.primitives.test.cpp:71:type_infer_primitives_check_methods_of_number`
//! Source: `tests/TypeInfer.primitives.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.primitives.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.primitives.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_primitives_check_methods_of_number

#[cfg(test)]
#[test]
fn type_infer_primitives_check_methods_of_number() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: number = 9999
        function x:y(z: number)
            local s: string = z
        end
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);

    if !luaur_common::FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Expected type table, got 'number' instead",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[1])
        );
    } else {
        assert_eq!(
            "Cannot add method to non-table type 'number'",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[1])
        );
    }
}
