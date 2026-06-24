//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Unifier2.test.cpp:123:unifier_2_unify_binds_free_supertype_tail_pack`
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
//!   - type_ref -> type_alias TypePackId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method TypePackFixture::freshTypePack (tests/TypePack.test.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item unifier_2_unify_binds_free_supertype_tail_pack

#[cfg(test)]
#[test]
fn unifier2_unify_binds_free_supertype_tail_pack() {
    use crate::records::unifier_2_fixture::Unifier2Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::type_pack::TypePack;

    let mut fixture = Unifier2Fixture::new();
    let number_pack = fixture
        .arena
        .add_type_pack_initializer_list_type_id(&[fixture.builtin_types.numberType]);

    let free_tail = fixture
        .arena
        .fresh_type_pack(&mut *fixture.scope, Polarity::Unknown);
    let free_head = fixture
        .arena
        .add_type(FreeType::free_type_scope_type_id_type_id_polarity(
            &mut *fixture.scope,
            fixture.builtin_types.neverType,
            fixture.builtin_types.unknownType,
            Polarity::Unknown,
        ));
    let free_and_free = fixture
        .arena
        .add_type_pack_t(TypePack::new(alloc::vec![free_head], Some(free_tail)));

    fixture.u2.unify_pack(number_pack, free_and_free);

    assert_eq!(
        "(number <: 'a)",
        fixture.to_string_type_pack_id(free_and_free)
    );
}
