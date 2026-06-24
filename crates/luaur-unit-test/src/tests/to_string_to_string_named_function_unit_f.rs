//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_to_string_named_function_unit_f() {
    use luaur_analysis::functions::to_string_named_function_to_string::to_string_named_function_string_function_type;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut arena = TypeArena::default();
    let empty_args = arena.add_type_pack_initializer_list_type_id(&[]);
    let empty_rets = arena.add_type_pack_initializer_list_type_id(&[]);
    let ftv = FunctionType::function_type_new(empty_args, empty_rets, None, false);

    assert_eq!(
        "f(): ()",
        to_string_named_function_string_function_type("f", &ftv)
    );
}
