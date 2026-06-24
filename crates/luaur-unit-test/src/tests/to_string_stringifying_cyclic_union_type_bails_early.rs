//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_stringifying_cyclic_union_type_bails_early() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::functions::get_mutable_type::get_mutable_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let string_type = fixture.get_builtins().stringType;
    let number_type = fixture.get_builtins().numberType;

    let mut arena = TypeArena::default();
    let tv = arena.add_type(UnionType {
        options: vec![string_type, number_type],
    });
    let utv =
        unsafe { get_mutable_type_id::<UnionType>(tv).as_mut() }.expect("expected union type");
    utv.options.push(tv);
    utv.options.push(tv);

    assert_eq!("t1 where t1 = number | string", to_string_type_id(tv));
}
