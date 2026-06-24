//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Generalization.test.cpp:112:generalization_dont_traverse_into_class_types_when_generalizing`
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
//!   - calls -> method GeneralizationFixture::freshType (tests/Generalization.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - calls -> method GeneralizationFixture::generalize (tests/Generalization.test.cpp)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item generalization_dont_traverse_into_class_types_when_generalizing

#[cfg(test)]
#[test]
fn generalization_dont_traverse_into_class_types_when_generalizing() {
    use crate::records::generalization_fixture::GeneralizationFixture;
    use alloc::collections::BTreeMap;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::property_type::Property;

    let mut fixture = GeneralizationFixture::new();
    let (prop_ty, _) = fixture.fresh_type();

    let mut props = BTreeMap::new();
    props.insert(String::from("oh_no"), Property::readonly(prop_ty));
    let cursed_extern_type = fixture.arena.add_type(ExternType {
        name: String::from("Cursed"),
        props,
        parent: None,
        metatable: None,
        tags: Default::default(),
        user_data: None,
        definition_module_name: Default::default(),
        definition_location: None,
        indexer: None,
        relation: None,
    });

    let gen_extern_type = fixture.generalize(cursed_extern_type);
    assert!(gen_extern_type.is_some());

    let gen_extern_type = gen_extern_type.unwrap();
    let extern_type = unsafe { get_type_id::<ExternType>(gen_extern_type).as_ref() }.unwrap();
    let gen_prop_ty = extern_type.props.get("oh_no").unwrap().read_ty.unwrap();
    assert!(!unsafe { get_type_id::<FreeType>(gen_prop_ty) }.is_null());
}
