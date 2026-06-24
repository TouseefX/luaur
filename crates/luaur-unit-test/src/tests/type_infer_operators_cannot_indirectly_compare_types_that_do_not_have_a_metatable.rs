//! Ported from `tests/TypeInfer.operators.test.cpp`.
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.operators.test.cpp:292:type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable`
//! Source: `tests/TypeInfer.operators.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.operators.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.operators.test.cpp
//! - outgoing:
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record CannotCompareUnrelatedTypes (Analysis/include/Luau/Error.h)
//!   - type_ref -> record GenericError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable

#[cfg(test)]
#[test]
fn type_infer_operators_cannot_indirectly_compare_types_that_do_not_have_a_metatable() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::cannot_compare_unrelated_types::CannotCompareUnrelatedTypes;
    use luaur_analysis::records::generic_error::GenericError;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a = {}
        local b = {}
        local c = a < b
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        type_error_data_ref::<CannotCompareUnrelatedTypes>(&result.errors[0])
            .expect("expected CannotCompareUnrelatedTypes");
    } else {
        let gen =
            type_error_data_ref::<GenericError>(&result.errors[0]).expect("expected GenericError");
        assert_eq!(
            "Type a cannot be compared with < because it has no metatable",
            gen.message()
        );
    }
}
