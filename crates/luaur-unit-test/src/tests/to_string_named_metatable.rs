//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_named_metatable() {
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::metatable_type::MetatableType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::table_type::TableType;

    let table = Type::from(TableType::table_type());
    let metatable = Type::from(TableType::table_type());
    let mtv = Type::from(MetatableType::new_named(
        &table as *const _,
        &metatable as *const _,
        String::from("NamedMetatable"),
    ));

    assert_eq!("NamedMetatable", to_string_type_item(&mtv));
}
