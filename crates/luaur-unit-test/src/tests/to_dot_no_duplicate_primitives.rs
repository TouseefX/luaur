//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:83:to_dot_no_duplicate_primitives`
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
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - type_ref -> record PrimitiveType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record AnyType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnknownType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record NeverType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item to_dot_no_duplicate_primitives

#[cfg(test)]
#[test]
fn to_dot_no_duplicate_primitives() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;

    let mut fixture = Fixture::default();
    let builtins = fixture.get_builtins();
    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: false,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"PrimitiveType number\"];\n}",
        to_dot(builtins.numberType, &opts)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"AnyType 1\"];\n}",
        to_dot(builtins.anyType, &opts)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"UnknownType 1\"];\n}",
        to_dot(builtins.unknownType, &opts)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"NeverType 1\"];\n}",
        to_dot(builtins.neverType, &opts)
    );
}
