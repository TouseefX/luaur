//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_stringifying_array_uses_array_syntax() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::enums::table_state::TableState;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::table_indexer::TableIndexer;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let mut ttv = TableType::table_type();
    ttv.state = TableState::Sealed;
    ttv.indexer = Some(TableIndexer {
        index_type: builtins.numberType,
        index_result_type: builtins.stringType,
        is_read_only: false,
    });

    assert_eq!("{string}", to_string_type_item(&Type::from(ttv.clone())));

    ttv.props
        .insert(String::from("A"), Property::rw_type_id(builtins.numberType));
    assert_eq!(
        "{ [number]: string, A: number }",
        to_string_type_item(&Type::from(ttv.clone()))
    );

    ttv.props.clear();
    ttv.state = TableState::Unsealed;
    assert_eq!("{string}", to_string_type_item(&Type::from(ttv)));
}
