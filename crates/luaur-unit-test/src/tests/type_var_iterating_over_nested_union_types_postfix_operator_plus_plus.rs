//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:91:type_var_iterating_over_nested_union_types_postfix_operator_plus_plus`
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
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_var_iterating_over_nested_union_types_postfix_operator_plus_plus

#[cfg(test)]
#[test]
fn type_var_iterating_over_nested_union_types_postfix_operator_plus_plus() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use alloc::vec::Vec;
    use luaur_analysis::functions::begin_type::begin_union_type;
    use luaur_analysis::functions::end_type::end_union_type;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let subunion = Type::from(UnionType {
        options: vec![builtins.numberType, builtins.stringType],
    });
    let utv = UnionType {
        options: vec![builtins.anyType, &subunion],
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(&utv);
    let end = end_union_type(&utv);
    while it.operator_ne(&end) {
        let mut old = it.operator_inc_i32();
        result.push(old.operator_deref());
    }

    assert_eq!(3, result.len());
    assert_eq!(builtins.anyType, result[0]);
    assert_eq!(builtins.stringType, result[2]);
    assert_eq!(builtins.numberType, result[1]);
}
