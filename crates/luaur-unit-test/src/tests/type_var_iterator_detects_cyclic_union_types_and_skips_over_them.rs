//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TypeVar.test.cpp:110:type_var_iterator_detects_cyclic_union_types_and_skips_over_them`
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
//!   - translates_to -> rust_item type_var_iterator_detects_cyclic_union_types_and_skips_over_them

#[cfg(test)]
#[test]
fn type_var_iterator_detects_cyclic_union_types_and_skips_over_them() {
    use crate::records::fixture::Fixture;
    use alloc::vec::Vec;
    use luaur_analysis::functions::begin_type::begin_union_type;
    use luaur_analysis::functions::end_type::end_union_type;
    use luaur_analysis::records::r#type::Type;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::type_variant::TypeVariant;

    let mut fixture = Fixture::fixture_bool(false);
    let builtins = fixture.get_builtins();

    let mut atv = Type::from(UnionType::default());
    let mut btv = Type::from(UnionType::default());
    let atv_id = &atv as *const Type;
    let btv_id = &btv as *const Type;

    if let TypeVariant::Union(utv2) = &mut btv.ty {
        utv2.options.push(builtins.numberType);
        utv2.options.push(builtins.stringType);
        utv2.options.push(atv_id);
    } else {
        unreachable!();
    }

    if let TypeVariant::Union(utv1) = &mut atv.ty {
        utv1.options.push(btv_id);
    } else {
        unreachable!();
    }

    let TypeVariant::Union(utv2) = &btv.ty else {
        unreachable!();
    };

    let mut result = Vec::new();
    let mut it = begin_union_type(utv2);
    let end = end_union_type(utv2);
    while it.operator_ne(&end) {
        result.push(it.operator_deref());
        it.operator_inc();
    }

    assert_eq!(2, result.len());
    assert_eq!(builtins.numberType, result[0]);
    assert_eq!(builtins.stringType, result[1]);
}
