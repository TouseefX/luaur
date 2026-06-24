//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_builtin_top_extern_types() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    assert_eq!("object", to_string_type_id(builtins.objectType));
    assert_eq!("class", to_string_type_id(builtins.classType));
}
