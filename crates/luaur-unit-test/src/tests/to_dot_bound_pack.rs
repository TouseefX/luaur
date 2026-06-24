//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:466:to_dot_bound_pack`
//! Source: `tests/ToDot.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/ToDot.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Scope.h
//!   - includes -> source_file Analysis/include/Luau/ToDot.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/ToDot.test.cpp
//! - outgoing:
//!   - type_ref -> record TypePackVar (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> type_alias TypePackVariant (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> type_alias BoundTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - translates_to -> rust_item to_dot_bound_pack

#[cfg(test)]
#[test]
fn to_dot_bound_pack() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot_alt_b::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_pack::TypePack;
    use luaur_analysis::type_aliases::bound_type_pack::BoundTypePack;

    let mut fixture = Fixture::default();
    let number_type = fixture.get_builtins().numberType;
    let mut arena = TypeArena::default();
    let pack = arena.add_type_pack_t(TypePack::new(alloc::vec![number_type], None));
    let bound = arena.add_type_pack_t(BoundTypePack::bound_t(pack));

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"BoundTypePack 1\"];\nn1 -> n2;\nn2 [label=\"TypePack 2\"];\nn2 -> n3;\nn3 [label=\"number\"];\n}",
        to_dot(bound, &opts)
    );
}
