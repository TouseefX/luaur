//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:6955:type_infer_tables_compound_assignment_writes_lhs`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record PropertyAccessViolation (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_compound_assignment_writes_lhs

#[cfg(test)]
#[test]
fn type_infer_tables_compound_assignment_writes_lhs() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::property_access_violation::PropertyAccessViolation;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_OLD_SOLVER_GUARD!();
    let _visit_lhs = ScopedFastFlag::new(&FFlag::LuauLValueCompoundAssignmentVisitLhs, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = {
            read x: number
        }

        local foo: T = { x = 5 }
        foo.x += 5
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<PropertyAccessViolation>(&result.errors[0])
        .expect("expected PropertyAccessViolation");
}
