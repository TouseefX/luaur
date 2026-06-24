//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:194:to_dot_union`
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
//!   - type_ref -> record CheckResult (Analysis/include/Luau/Frontend.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item to_dot_union

#[cfg(test)]
#[test]
fn to_dot_union() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local a: string | number
"#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"UnionType 1\"];\nn1 -> n2;\nn2 [label=\"string\"];\nn1 -> n3;\nn3 [label=\"number\"];\n}",
        to_dot(fixture.require_type_string(&String::from("a")), &opts)
    );
}
