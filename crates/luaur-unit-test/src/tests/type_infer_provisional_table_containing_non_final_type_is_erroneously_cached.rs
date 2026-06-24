//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeInfer.provisional.test.cpp:1252:type_infer_provisional_table_containing_non_final_type_is_erroneously_cached`
//! Source: `tests/TypeInfer.provisional.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeInfer.provisional.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/RecursionCounter.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TypeInfer.provisional.test.cpp
//! - outgoing:
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record UnifierSharedState (Analysis/include/Luau/UnifierSharedState.h)
//!   - type_ref -> record Normalizer (Analysis/include/Luau/Normalize.h)
//!   - type_ref -> enum SolverMode (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item type_infer_provisional_table_containing_non_final_type_is_erroneously_cached

#[cfg(test)]
#[test]
fn type_infer_provisional_table_containing_non_final_type_is_erroneously_cached() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use alloc::string::String;
    use alloc::sync::Arc;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = NormalizeFixture::default();
    let scope = fixture.get_global_scope();
    let builtins = fixture.base.builtin_types;

    let mut table = TableType::table_type();
    let free_ty = unsafe {
        fixture
            .arena
            .fresh_type_not_null_builtin_types_scope(&*builtins, scope)
    };
    table
        .props
        .insert(String::from("foo"), Property::rw_type_id(free_ty));
    let table_ty = fixture.arena.add_type(table);

    let n1 = fixture
        .normalize(table_ty)
        .expect("expected normalized table");
    let n2 = fixture
        .normalize(table_ty)
        .expect("expected normalized table");

    assert!(Arc::ptr_eq(&n1, &n2));
}
