//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.tryUnify.test.cpp:86:type_infer_try_unify_tables_can_be_unified`
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
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method Property::type_DEPRECATED (Analysis/src/Type.cpp)
//!   - translates_to -> rust_item type_infer_try_unify_tables_can_be_unified

#[cfg(test)]
#[test]
fn type_infer_try_unify_tables_can_be_unified() {
    use crate::records::try_unify_fixture::TryUnifyFixture;
    use luaur_analysis::functions::follow_type::follow_type_id;

    let mut fixture = TryUnifyFixture::default();
    let foo_one = fixture.fresh_type();
    let foo_two = fixture.fresh_type();
    let table_one = fixture.unsealed_table_type(&[("foo", foo_one)]);
    let table_two = fixture.unsealed_table_type(&[("foo", foo_two)]);

    let table_one_foo = fixture.table_prop_type(table_one, "foo");
    let table_two_foo = fixture.table_prop_type(table_two, "foo");
    assert_ne!(table_one_foo, table_two_foo);

    fixture
        .state
        .try_unify_type_id_type_id_bool_bool_literal_properties(
            table_two, table_one, false, false, None,
        );

    assert!(!fixture.state.failure);
    assert!(
        fixture.state.errors.is_empty(),
        "{:?}",
        fixture.state.errors
    );

    fixture.state.log.commit();

    assert_eq!(unsafe { follow_type_id(table_one_foo) }, unsafe {
        follow_type_id(table_two_foo)
    });
}
