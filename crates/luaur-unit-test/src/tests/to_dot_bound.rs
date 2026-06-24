//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:118:to_dot_bound`
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
//!   - type_ref -> record TypeArena (Analysis/include/Luau/TypeArena.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> type_alias BoundType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - translates_to -> rust_item to_dot_bound

#[cfg(test)]
#[test]
fn to_dot_bound() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::type_aliases::bound_type::BoundType;

    let mut fixture = Fixture::default();
    let number_type = fixture.get_builtins().numberType;
    let mut arena = TypeArena::default();
    let ty = arena.add_type(BoundType::bound_t(number_type));

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"BoundType 1\"];\nn1 -> n2;\nn2 [label=\"number\"];\n}",
        to_dot(ty, &opts)
    );
}
