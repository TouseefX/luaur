//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_the_bound_to_table_type_contained_within_a_type_pack() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_d::to_string_type_pack_id;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().numberType;
    let mut arena = TypeArena::default();

    let mut table = TableType::table_type();
    table.state = TableState::Sealed;
    table
        .props
        .insert(String::from("hello"), Property::rw_type_id(number_type));
    table
        .props
        .insert(String::from("world"), Property::rw_type_id(number_type));
    let tv1 = arena.add_type(table);
    let tpv1 = arena.add_type_pack_initializer_list_type_id(&[tv1]);

    let mut bound_table = TableType::table_type();
    bound_table.state = TableState::Free;
    bound_table
        .props
        .insert(String::from("hello"), Property::rw_type_id(number_type));
    let tv2 = arena.add_type(bound_table);
    let bttv =
        unsafe { get_mutable_type_id::<TableType>(tv2).as_mut() }.expect("expected table type");
    bttv.bound_to = Some(tv1);
    let tpv2 = arena.add_type_pack_initializer_list_type_id(&[tv2]);

    assert_eq!(
        "{ hello: number, world: number }",
        to_string_type_pack_id(tpv1)
    );
    assert_eq!(
        "{ hello: number, world: number }",
        to_string_type_pack_id(tpv2)
    );
}
