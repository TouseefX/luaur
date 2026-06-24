//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_the_empty_type_pack_should_be_parenthesized() {
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::functions::to_string_to_string_alt_g::to_string_type_pack_var;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::records::type_pack_var::TypePackVar;

    let empty_type_pack = TypePackVar::from(TypePack::new(vec![], None));
    assert_eq!("()", to_string_type_pack_var(&empty_type_pack));

    let mut arena = TypeArena::default();
    let empty_arg_pack = arena.add_type_pack_initializer_list_type_id(&[]);
    let empty_ret_pack = arena.add_type_pack_initializer_list_type_id(&[]);
    let unit_to_unit = Type::from(FunctionType::function_type_new(
        empty_arg_pack,
        empty_ret_pack,
        None,
        false,
    ));
    assert_eq!("() -> ()", to_string_type_item(&unit_to_unit));
}
