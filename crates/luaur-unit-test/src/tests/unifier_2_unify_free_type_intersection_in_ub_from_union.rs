//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:136:unifier_2_unify_free_type_intersection_in_ub_from_union`
//! Source: `tests/Unifier2.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Unifier2.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/TypeArena.h
//!   - includes -> source_file Analysis/include/Luau/Unifier2.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Unifier2.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item unifier_2_unify_free_type_intersection_in_ub_from_union

#[cfg(test)]
#[test]
fn unifier2_unify_free_type_intersection_in_ub_from_union() {
    use crate::records::unifier_2_fixture::Unifier2Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = Unifier2Fixture::new();

    let free_ty = fixture
        .arena
        .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
            &mut *fixture.scope,
            fixture.builtin_types.neverType,
            fixture.builtin_types.unknownType,
            Polarity::Unknown,
        ));
    let sub_ty = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![free_ty, fixture.builtin_types.truthyType],
    });
    let super_ty = fixture.arena.add_type(UnionType {
        options: alloc::vec![
            fixture.builtin_types.numberType,
            fixture.builtin_types.nilType
        ],
    });

    fixture.u2.unify(sub_ty, super_ty);

    assert_eq!("('a <: never)", fixture.to_string_type_id(free_ty));
}
