//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:5034:type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop`
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
//!   - type_ref -> record FunctionExitsWithoutReturning (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop

#[cfg(test)]
#[test]
fn type_infer_tables_insert_a_and_f_of_a_into_table_res_in_a_loop() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(t)
            local res = {}

            for k, a in t do
                res[k] = f(a)
                res[k] = a
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
            .expect("expected FunctionExitsWithoutReturning");
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    }
}
