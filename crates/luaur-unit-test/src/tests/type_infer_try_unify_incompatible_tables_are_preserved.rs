//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:111:type_infer_try_unify_incompatible_tables_are_preserved`
//! Source: `tests/TypeInfer.tryUnify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.tryUnify.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Symbol.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.tryUnify.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method Property::type_DEPRECATED (Analysis/src/Type.cpp)
//!   - translates_to -> rust_item type_infer_try_unify_incompatible_tables_are_preserved

#[cfg(test)]
#[test]
fn type_infer_try_unify_incompatible_tables_are_preserved() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use luaur_analysis::functions::follow_type::follow_type_id;

    let mut fixture = TryUnifyFixture::default();
    let number_type = fixture.get_builtins().numberType;
    let string_type = fixture.get_builtins().stringType;
    let foo_one = fixture.fresh_type();
    let foo_two = fixture.fresh_type();
    let table_one = fixture.unsealed_table_type(&[("foo", foo_one), ("bar", number_type)]);
    let table_two = fixture.unsealed_table_type(&[("foo", foo_two), ("bar", string_type)]);

    let table_one_foo = fixture.table_prop_type(table_one, "foo");
    let table_two_foo = fixture.table_prop_type(table_two, "foo");
    assert_ne!(table_one_foo, table_two_foo);

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(
            table_two, table_one, false, false, None,
        );

    assert!(fixture.state.failure);
    assert_eq!(1, fixture.state.errors.len(), "{:?}", fixture.state.errors);
    assert_ne!(unsafe { follow_type_id(table_one_foo) }, unsafe {
        follow_type_id(table_two_foo)
    });
}
