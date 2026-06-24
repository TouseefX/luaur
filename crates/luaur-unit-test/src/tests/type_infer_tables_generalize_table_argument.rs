//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:3027:type_infer_tables_generalize_table_argument`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item type_infer_tables_generalize_table_argument

#[cfg(test)]
#[test]
fn type_infer_tables_generalize_table_argument() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::first::first;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::table_type::TableType;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo(arr)
            local work = {}
            for i = 1, #arr do
                work[i] = arr[i]
            end

            return arr
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let foo_type = fixture.require_type_string(&String::from("foo"));
    let foo_type = unsafe { get_type_id::<FunctionType>(foo_type).as_ref() }
        .expect("expected foo FunctionType");

    let foo_arg1 = first(foo_type.arg_types(), false).expect("expected first foo argument");
    let foo_arg1_table = unsafe { get_type_id::<TableType>(follow_type_id(foo_arg1)).as_ref() }
        .expect("expected foo first argument TableType");

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(TableState::Sealed, foo_arg1_table.state);
    } else {
        assert_eq!(TableState::Generic, foo_arg1_table.state);
    }
}
