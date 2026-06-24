//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_no_parentheses_around_return_type_if_pack_has_an_empty_head_link() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let string_type = fixture.get_builtins().stringType;
    let mut arena = TypeArena::default();

    let real_tail = arena.add_type_pack_initializer_list_type_id(&[string_type]);
    let empty_tail =
        arena.add_type_pack_vector_type_id_optional_type_pack_id(vec![], Some(real_tail));
    let arg_list = arena.add_type_pack_initializer_list_type_id(&[string_type]);

    let function_type = arena.add_type(FunctionType::function_type_new(
        arg_list, empty_tail, None, false,
    ));

    assert_eq!("(string) -> string", to_string_type_id(function_type));
}
