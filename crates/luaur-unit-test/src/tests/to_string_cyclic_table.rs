//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_cyclic_table() {
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut arena = TypeArena::default();
    let cyclic_table = arena.add_type(TableType::table_type());
    let table_one = unsafe { get_mutable_type_id::<TableType>(cyclic_table).as_mut() }
        .expect("expected cyclic table");
    table_one
        .props
        .insert("self".into(), Property::rw_type_id(cyclic_table));

    assert_eq!(
        "t1 where t1 = {| self: t1 |}",
        to_string_type_id(cyclic_table)
    );
}
