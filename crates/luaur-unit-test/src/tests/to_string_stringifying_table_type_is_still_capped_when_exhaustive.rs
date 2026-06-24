//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_stringifying_table_type_is_still_capped_when_exhaustive() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::property_type::Property;
    use luaur_analysis::records::table_type::TableType;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().numberType;
    let mut table = TableType::table_type();
    for c in b'a'..=b'g' {
        table.props.insert(
            String::from(char::from(c)),
            Property::rw_type_id(number_type),
        );
    }

    let mut arena = TypeArena::default();
    let tv = arena.add_type(table);

    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    opts.max_table_length = 40;
    assert_eq!(
        "{| a: number, b: number, c: number, d: number, e: number, ... 2 more ... |}",
        to_string_type_id_to_string_options(tv, &mut opts)
    );
}
