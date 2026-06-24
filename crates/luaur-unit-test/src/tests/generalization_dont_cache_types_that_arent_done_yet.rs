//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:141:generalization_dont_cache_types_that_arent_done_yet`
//! Source: `tests/Generalization.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Generalization.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Generalization.h
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Generalization.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_dont_cache_types_that_arent_done_yet

#[cfg(test)]
#[test]
fn generalization_dont_cache_types_that_arent_done_yet() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use alloc::collections::BTreeMap;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::scope::Scope;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;
    use std::sync::Arc;

    let mut fixture = GeneralizationFixture::new();
    let global_scope = Arc::as_ptr(&fixture.global_scope) as *mut Scope;
    let free_ty = fixture
        .arena
        .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
            global_scope,
            fixture.builtin_types.neverType,
            fixture.builtin_types.stringType,
            Polarity::Unknown,
        ));

    let fn_ret = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.numberType]);
    let fn_ty = fixture.arena.add_type(FunctionType::function_type_new(
        fixture.builtin_types.emptyTypePack,
        fn_ret,
        None,
        false,
    ));

    let mut props = BTreeMap::new();
    props.insert(
        String::from("one"),
        Property::rw_type_id(fixture.builtin_types.numberType),
    );
    props.insert(String::from("two"), Property::rw_type_id(free_ty));
    props.insert(String::from("three"), Property::rw_type_id(fn_ty));
    let table_ty = fixture.arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );

    fixture.generalize(table_ty);

    assert!(fixture.generalized_types.contains(&fn_ty));
    assert!(fixture
        .generalized_types
        .contains(&fixture.builtin_types.numberType));
    assert!(fixture
        .generalized_types
        .contains(&fixture.builtin_types.neverType));
    assert!(fixture
        .generalized_types
        .contains(&fixture.builtin_types.stringType));
    assert!(!fixture.generalized_types.contains(&free_ty));
    assert!(!fixture.generalized_types.contains(&table_ty));
}
