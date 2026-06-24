//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_self_recursive_instantiated_param() {
    use alloc::string::String;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut arena = TypeArena::default();
    let table_ty = arena.add_type(TableType::table_type());
    let ttv =
        unsafe { get_mutable_type_id::<TableType>(table_ty).as_mut() }.expect("expected table");
    ttv.name = Some(String::from("Table"));
    ttv.instantiated_type_params.push(table_ty);

    assert_eq!("Table<Table>", to_string_type_id(table_ty));
}
