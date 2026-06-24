//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_cycle_rooted_in_a_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use alloc::vec;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::get_mutable_type_pack::get_mutable_type_pack_id;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_level::TypeLevel;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::type_aliases::props_type::Props;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().numberType;
    let unknown_type = fixture.get_builtins().unknownType;
    let mut arena = TypeArena::default();

    let the_pack = arena.add_type_pack_initializer_list_type_id(&[number_type, number_type]);
    let empty_pack = arena.add_type_pack_initializer_list_type_id(&[]);
    let base_method = arena.add_type(FunctionType::function_type_new(
        the_pack, empty_pack, None, false,
    ));

    let mut props = Props::new();
    props.insert(String::from("BaseField"), Property::readonly(unknown_type));
    props.insert(String::from("BaseMethod"), Property::readonly(base_method));

    let the_table = arena.add_type(
        TableType::table_type_props_optional_table_indexer_type_level_table_state(
            &props,
            None,
            TypeLevel::default(),
            TableState::Sealed,
        ),
    );

    let pack_ptr =
        unsafe { get_mutable_type_pack_id::<TypePack>(the_pack).as_mut() }.expect("expected pack");
    pack_ptr.head_mut()[0] = the_table;

    assert_eq!(
        "tp1 where tp1 = { read BaseField: unknown, read BaseMethod: (tp1) -> () }, number",
        to_string_type_pack_id(the_pack)
    );
}
