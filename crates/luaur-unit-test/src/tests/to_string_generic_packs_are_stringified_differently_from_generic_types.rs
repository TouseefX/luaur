//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_generic_packs_are_stringified_differently_from_generic_types() {
    use alloc::string::String;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::functions::to_string_to_string_alt_g::to_string_type_pack_var;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::generic_type_pack::GenericTypePack;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_pack_var::TypePackVar;

    let tpv = TypePackVar::from(GenericTypePack::new_name(String::from("a")));
    assert_eq!("a...", to_string_type_pack_var(&tpv));

    let tv = Type::from(GenericType::generic_type_name_polarity(
        &String::from("a"),
        Polarity::Mixed,
    ));
    assert_eq!("a", to_string_type_item(&tv));
}
