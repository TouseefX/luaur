//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:336:to_dot_free_with_constraints`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> type_alias TypeVariant (Analysis/include/Luau/Type.h)
//!   - type_ref -> record FreeType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item to_dot_free_with_constraints

#[cfg(test)]
#[test]
fn to_dot_free_with_constraints() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::free_type::FreeType;
    use luaur_analysis::records::to_dot_options::ToDotOptions;
    use luaur_analysis::records::type_arena::TypeArena;

    let mut fixture = Fixture::default();
    let builtins = fixture.get_builtins();
    let mut arena = TypeArena::default();
    let ty = arena.add_type(FreeType::free_type_scope_type_id_type_id_polarity(
        core::ptr::null_mut(),
        builtins.numberType,
        builtins.optionalNumberType,
        Polarity::Unknown,
    ));

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"FreeType 1\"];\nn1 -> n2 [label=\"[lowerBound]\"];\nn2 [label=\"number\"];\nn1 -> n3 [label=\"[upperBound]\"];\nn3 [label=\"UnionType 3\"];\nn3 -> n4;\nn4 [label=\"number\"];\nn3 -> n5;\nn5 [label=\"nil\"];\n}",
        to_dot(ty, &opts)
    );
}
