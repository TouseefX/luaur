//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:374:to_dot_generic`
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
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - type_ref -> enum Polarity (Analysis/include/Luau/Polarity.h)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - translates_to -> rust_item to_dot_generic

#[cfg(test)]
#[test]
fn to_dot_generic() {
    use luaur_analysis::enums::polarity::Polarity;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::generic_type::GenericType;
    use luaur_analysis::records::to_dot_options::ToDotOptions;
    use luaur_analysis::records::type_arena::TypeArena;
    use luaur_analysis::records::type_level::TypeLevel;

    let mut arena = TypeArena::default();
    let ty = arena.add_type(GenericType {
        index: 0,
        level: TypeLevel::default(),
        scope: core::ptr::null_mut(),
        name: "T".to_string(),
        explicit_name: true,
        polarity: Polarity::Mixed,
    });

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"GenericType T\"];\n}",
        to_dot(ty, &opts)
    );
}
