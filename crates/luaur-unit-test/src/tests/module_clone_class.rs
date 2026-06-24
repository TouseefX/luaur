//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:233:module_clone_class`
//! Source: `tests/Module.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Module.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Clone.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Module.test.cpp
//! - outgoing:
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record Test (tests/NotNull.test.cpp)
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> record CloneState (Analysis/include/Luau/Clone.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - translates_to -> rust_item module_clone_class

#[cfg(test)]
#[test]
fn module_clone_class() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::clone_clone_alt_b::clone as clone_type;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::clone_state::CloneState;
    use luaur_analysis::records::extern_type::ExternType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::props_type_alt_c::Props;

    let mut fixture = Fixture::fixture_bool(false);
    let any_type = fixture.get_builtins().anyType;
    let number_type = fixture.get_builtins().numberType;
    let string_type = fixture.get_builtins().stringType;

    let mut metaclass_props = Props::default();
    metaclass_props.insert("__add".into(), Property::readonly(any_type));
    let example_meta_class = Type::from(ExternType {
        name: "ExampleClassMeta".into(),
        props: metaclass_props,
        parent: None,
        metatable: None,
        tags: Default::default(),
        user_data: None,
        definition_module_name: "Test".into(),
        definition_location: None,
        indexer: None,
        relation: None,
    });
    let example_meta_class_id = &example_meta_class as *const Type;

    let mut class_props = Props::default();
    class_props.insert("PropOne".into(), Property::readonly(number_type));
    class_props.insert("PropTwo".into(), Property::readonly(string_type));
    let example_class = Type::from(ExternType {
        name: "ExampleClass".into(),
        props: class_props,
        parent: None,
        metatable: Some(example_meta_class_id),
        tags: Default::default(),
        user_data: None,
        definition_module_name: "Test".into(),
        definition_location: None,
        indexer: None,
        relation: None,
    });
    let example_class_id = &example_class as *const Type;

    let mut dest = TypeArena::default();
    let mut clone_state = CloneState::new(fixture.get_builtins());

    let cloned = clone_type(example_class_id, &mut dest, &mut clone_state);
    let etv = unsafe { get_type_id::<ExternType>(cloned).as_ref() }.expect("expected extern type");

    let metatable_ty = etv.metatable.expect("expected cloned metatable");
    let metatable = unsafe { get_type_id::<ExternType>(metatable_ty).as_ref() }
        .expect("expected extern metatable");

    assert_eq!("ExampleClass", etv.name);
    assert_eq!("ExampleClassMeta", metatable.name);
}
