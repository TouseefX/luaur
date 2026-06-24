//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:161:generalization_functions_containing_cyclic_tables_can_be_cached`
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
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - calls -> method Variant::emplace (Common/include/Luau/Variant.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record TypeLevel (Analysis/include/Luau/Unifiable.h)
//!   - type_ref -> enum TableState (Analysis/include/Luau/Type.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - translates_to -> rust_item generalization_functions_containing_cyclic_tables_can_be_cached

#[cfg(test)]
#[test]
fn generalization_functions_containing_cyclic_tables_can_be_cached() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use alloc::collections::BTreeMap;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::as_mutable_type::as_mutable_type_id;
    use luaur_analysis::records::blocked_type::BlockedType;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = GeneralizationFixture::new();
    let self_ty = fixture.arena.add_type(BlockedType::default());

    let method_args = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[self_ty]);
    let method_rets = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.numberType]);
    let method_ty = fixture.arena.add_type(FunctionType::function_type_new(
        method_args,
        method_rets,
        None,
        false,
    ));

    let mut props = BTreeMap::new();
    props.insert(
        String::from("count"),
        Property::rw_type_id(fixture.builtin_types.numberType),
    );
    props.insert(String::from("method"), Property::rw_type_id(method_ty));
    unsafe {
        (*as_mutable_type_id(self_ty)).ty = TypeVariant::Table(
            TableType::table_type_props_optional_table_indexer_type_level_table_state(
                &props,
                None,
                TypeLevel::default(),
                TableState::Sealed,
            ),
        );
    }

    fixture.generalize(method_ty);

    assert!(fixture.generalized_types.contains(&method_ty));
    assert!(fixture.generalized_types.contains(&self_ty));
    assert!(fixture
        .generalized_types
        .contains(&fixture.builtin_types.numberType));
}
