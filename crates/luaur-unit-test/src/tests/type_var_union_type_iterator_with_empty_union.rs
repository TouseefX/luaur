//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:159:type_var_union_type_iterator_with_empty_union`
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
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - translates_to -> rust_item type_var_union_type_iterator_with_empty_union

#[cfg(test)]
#[test]
fn type_var_union_type_iterator_with_empty_union() {
    use alloc::vec::Vec;
    use luaur_analysis::functions::begin_type::begin_union_type;
    use luaur_analysis::functions::end_type::end_union_type;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let tv = Type::from(UnionType::default());
    let TypeVariant::Union(utv) = &tv.ty else {
        unreachable!();
    };

    let mut actual = Vec::new();
    let mut it = begin_union_type(utv);
    let end = end_union_type(utv);
    while it.operator_ne(&end) {
        actual.push(it.operator_deref());
        it.operator_inc();
    }

    assert!(actual.is_empty());
}
