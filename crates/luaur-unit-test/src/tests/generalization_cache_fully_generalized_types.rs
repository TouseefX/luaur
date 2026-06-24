//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:126:generalization_cache_fully_generalized_types`
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
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_cache_fully_generalized_types

#[cfg(test)]
#[test]
fn generalization_cache_fully_generalized_types() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use alloc::collections::BTreeMap;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut fixture = GeneralizationFixture::new();
    assert!(fixture.generalized_types.empty());

    let mut props = BTreeMap::new();
    props.insert(
        String::from("one"),
        Property::rw_type_id(fixture.builtin_types.numberType),
    );
    props.insert(
        String::from("two"),
        Property::rw_type_id(fixture.builtin_types.stringType),
    );
    let tiny_table = fixture.arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );

    fixture.generalize(tiny_table);

    assert!(fixture.generalized_types.contains(&tiny_table));
    assert!(fixture
        .generalized_types
        .contains(&fixture.builtin_types.numberType));
    assert!(fixture
        .generalized_types
        .contains(&fixture.builtin_types.stringType));
}
