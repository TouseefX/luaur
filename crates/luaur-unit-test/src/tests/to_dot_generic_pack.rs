//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:444:to_dot_generic_pack`
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
//!   - type_ref -> record GenericTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - translates_to -> rust_item to_dot_generic_pack

#[cfg(test)]
#[test]
fn to_dot_generic_pack() {
    use luaur_analysis::functions::to_dot_to_dot_alt_b::to_dot;
    use luaur_analysis::records::generic_type_pack::GenericTypePack;
    use luaur_analysis::records::to_dot_options::ToDotOptions;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut arena = TypeArena::default();
    let pack1 = arena.add_type_pack_t(GenericTypePack::new());
    let pack2 = arena.add_type_pack_t(GenericTypePack::new_name("T".to_string()));

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"GenericTypePack 1\"];\n}",
        to_dot(pack1, &opts)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"GenericTypePack T\"];\n}",
        to_dot(pack2, &opts)
    );
}
