//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_functions_are_always_parenthesized_in_unions_or_intersections() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let mut arena = TypeArena::default();

    let string_and_number_pack =
        arena.add_type_pack_initializer_list_type_id(&[builtins.stringType, builtins.numberType]);
    let number_and_string_pack =
        arena.add_type_pack_initializer_list_type_id(&[builtins.numberType, builtins.stringType]);

    let sn_to_ns = Type::from(FunctionType::function_type_new(
        string_and_number_pack,
        number_and_string_pack,
        None,
        false,
    ));
    let ns_to_sn = Type::from(FunctionType::function_type_new(
        number_and_string_pack,
        string_and_number_pack,
        None,
        false,
    ));

    let utv = Type::from(UnionType {
        options: vec![&ns_to_sn as *const _, &sn_to_ns as *const _],
    });
    let itv = Type::from(IntersectionType {
        parts: vec![&ns_to_sn as *const _, &sn_to_ns as *const _],
    });

    assert_eq!(
        "((number, string) -> (string, number)) | ((string, number) -> (number, string))",
        to_string_type_item(&utv)
    );
    assert_eq!(
        "((number, string) -> (string, number)) & ((string, number) -> (number, string))",
        to_string_type_item(&itv)
    );
}
