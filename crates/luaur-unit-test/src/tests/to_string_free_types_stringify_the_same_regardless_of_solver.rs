//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_free_types_stringify_the_same_regardless_of_solver() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    let mut arena = TypeArena::default();
    let t = arena.add_type(FreeType::free_type_scope_type_id_type_id_polarity(
        core::ptr::null_mut(),
        builtins.neverType,
        builtins.unknownType,
        Polarity::Unknown,
    ));

    assert_eq!("'a", to_string_type_id(t));
}
