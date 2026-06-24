//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.externTypes.test.cpp:387:type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties`
//! Source: `tests/TypeInfer.externTypes.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.externTypes.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.externTypes.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - translates_to -> rust_item type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties

#[cfg(test)]
#[test]
fn type_infer_extern_types_table_class_unification_reports_sane_errors_for_missing_properties() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(bar)
            bar.Y = 1 -- valid
            bar.x = 2 -- invalid, wanted 'X'
            bar.w = 2 -- invalid
        end

        local a: Vector2
        foo(a)
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Expected this to be '{ Y: number, w: number, x: number }', but got 'Vector2'",
            to_string_type_error(&result.errors[0])
        );
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Key 'w' not found in external type 'Vector2'",
            to_string_type_error(&result.errors[0])
        );
        assert_eq!(
            "Key 'x' not found in external type 'Vector2'.  Did you mean 'X'?",
            to_string_type_error(&result.errors[1])
        );
    }
}
