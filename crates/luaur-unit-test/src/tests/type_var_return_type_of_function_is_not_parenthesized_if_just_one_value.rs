//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:17:type_var_return_type_of_function_is_not_parenthesized_if_just_one_value`
//! Source: `tests/TypeVar.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TypeVar.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeInfer.h
//!   - includes -> source_file Analysis/include/Luau/VisitType.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/TypeVar.test.cpp
//! - outgoing:
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item type_var_return_type_of_function_is_not_parenthesized_if_just_one_value

#[cfg(test)]
#[test]
fn type_var_return_type_of_function_is_not_parenthesized_if_just_one_value() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use luaur_analysis::functions::to_string_to_string_alt_f::to_string_type_item;
    use luaur_analysis::records::function_type::FunctionType;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::records::type_pack_var::TypePackVar;

    let mut fixture = Fixture::fixture_bool(false);
    let number_type = fixture.get_builtins().numberType;

    let empty_argument_pack = TypePackVar::from(TypePack::new(vec![], None));
    let return_pack = TypePackVar::from(TypePack::new(vec![number_type], None));
    let returns_one = Type::from(FunctionType::function_type_new(
        &empty_argument_pack,
        &return_pack,
        None,
        false,
    ));

    let res = to_string_type_item(&returns_one);
    assert_eq!("() -> number", res);
}
