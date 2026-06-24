//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tables.test.cpp:4024:type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type`
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
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - type_ref -> record OptionalValueAccess (Analysis/include/Luau/Error.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type

#[cfg(test)]
#[test]
fn type_infer_tables_when_augmenting_an_unsealed_table_with_an_indexer_apply_the_correct_scope_to_the_indexer_type(
) {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::optional_value_access::OptionalValueAccess;
    use luaur_analysis::records::table_type::TableType;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local events = {}
        local mockObserveEvent = function(_, key, callback)
            events[key] = callback
        end

        events['FriendshipNotifications']({
            EventArgs = {
                UserId2 = '2'
            },
            Type = 'FriendshipDeclined'
        })
    "#,
        ),
        None,
    );

    let ty = unsafe { follow_type_id(fixture.require_type_string(&String::from("events"))) };
    let tt = unsafe { get_type_id::<TableType>(ty).as_ref() }
        .unwrap_or_else(|| panic!("expected table but got {}", to_string_type_id(ty)));

    assert!(tt.props.is_empty());
    let indexer = tt.indexer.expect("expected indexer");

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!("unknown", to_string_type_id(indexer.index_type));
        type_error_data_ref::<OptionalValueAccess>(&result.errors[0])
            .expect("expected OptionalValueAccess");
    } else {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
        assert_eq!("string", to_string_type_id(indexer.index_type));
    }
}
