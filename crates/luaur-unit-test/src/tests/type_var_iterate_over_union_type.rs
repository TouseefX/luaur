//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:60:type_var_iterate_over_union_type`
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
//!   - translates_to -> rust_item type_var_iterate_over_union_type

#[cfg(test)]
#[test]
fn type_var_iterate_over_union_type() {
    use crate::records::fixture::Fixture;
    use alloc::vec;
    use alloc::vec::Vec;
    use luaur_analysis::functions::begin_type::begin_union_type;
    use luaur_analysis::functions::end_type::end_union_type;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();
    let utv = UnionType {
        options: vec![builtins.numberType, builtins.stringType, builtins.anyType],
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(&utv);
    let end = end_union_type(&utv);
    while it.operator_ne(&end) {
        result.push(it.operator_deref());
        it.operator_inc();
    }

    assert_eq!(utv.options, result);
}
