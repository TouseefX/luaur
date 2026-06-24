//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:7062:type_infer_tables_table_read_any_counts_as_read_nil`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - translates_to -> rust_item type_infer_tables_table_read_any_counts_as_read_nil

#[cfg(test)]
#[test]
fn type_infer_tables_table_read_any_counts_as_read_nil() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _missing_properties_as_nil =
        ScopedFastFlag::new(&FFlag::LuauSubtypingMissingPropertiesAsNil, true);
    let _better_error_suppression =
        ScopedFastFlag::new(&FFlag::LuauSubtypingTablesHasBetterErrorSuppression, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(t: {}): { read foo: any }
            return t
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
