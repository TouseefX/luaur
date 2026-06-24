//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:150:unifier_2_unify_free_type_lb_from_intersection`
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
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record NegationType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record StringSingleton (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item unifier_2_unify_free_type_lb_from_intersection

#[cfg(test)]
#[test]
fn unifier2_unify_free_type_lb_from_intersection() {
    use crate::records::unifier_2_fixture::Unifier2Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::negation_type::NegationType;
    use luaur_analysis::records::singleton_type::SingletonType;
    use luaur_analysis::records::string_singleton::StringSingleton;
    use luaur_analysis::records::union_type::UnionType;
    use luaur_analysis::type_aliases::singleton_variant::SingletonVariant;

    let mut fixture = Unifier2Fixture::new();

    let free_ty = fixture
        .arena
        .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
            &mut *fixture.scope,
            fixture.builtin_types.neverType,
            fixture.builtin_types.unknownType,
            Polarity::Unknown,
        ));
    let super_ty = fixture.arena.add_type(UnionType {
        options: alloc::vec![free_ty, fixture.builtin_types.nilType],
    });
    let foo_singleton =
        fixture
            .arena
            .add_type(SingletonType::singleton_type(SingletonVariant::V1(
                StringSingleton::new("foo".into()),
            )));
    let not_foo = fixture.arena.add_type(NegationType::new(foo_singleton));
    let sub_ty = fixture.arena.add_type(IntersectionType {
        parts: alloc::vec![fixture.builtin_types.stringType, not_foo],
    });

    fixture.u2.unify(sub_ty, super_ty);

    assert_eq!(
        "(string & ~\"foo\" <: 'a)",
        fixture.to_string_type_id(free_ty)
    );
}
