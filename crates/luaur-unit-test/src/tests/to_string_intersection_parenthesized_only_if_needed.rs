//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_intersection_parenthesized_only_if_needed() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let utv = Type::from(UnionType {
        options: vec![builtins.numberType, builtins.stringType],
    });
    let itv = Type::from(IntersectionType {
        parts: vec![&utv as *const _, builtins.booleanType],
    });

    assert_eq!("(number | string) & boolean", to_string_type_item(&itv));
}
