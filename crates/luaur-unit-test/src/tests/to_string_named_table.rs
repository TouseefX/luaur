//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_named_table() {
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::table_type::TableType;

    let mut table = TableType::table_type();
    table.name = Some(String::from("TheTable"));
    let table = Type::from(table);

    assert_eq!("TheTable", to_string_type_item(&table));
}
