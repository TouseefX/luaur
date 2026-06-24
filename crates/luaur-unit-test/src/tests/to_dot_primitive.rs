//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:45:to_dot_primitive`
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
//!   - translates_to -> rust_item to_dot_primitive

#[cfg(test)]
#[test]
fn to_dot_primitive() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot_alt_c::to_dot;

    let mut fixture = Fixture::default();
    let builtins = fixture.get_builtins();

    assert_eq!(
        "digraph graphname {\nn1 [label=\"nil\"];\n}",
        to_dot(builtins.nilType)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"number\"];\n}",
        to_dot(builtins.numberType)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"any\"];\n}",
        to_dot(builtins.anyType)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"unknown\"];\n}",
        to_dot(builtins.unknownType)
    );
    assert_eq!(
        "digraph graphname {\nn1 [label=\"never\"];\n}",
        to_dot(builtins.neverType)
    );
}
