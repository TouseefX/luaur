//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:510:to_dot_builtintypes`
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
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record SingletonType (Analysis/include/Luau/Type.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item to_dot_builtintypes

#[cfg(test)]
#[test]
fn to_dot_builtintypes() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local x: "hi" | "\"hello\"" | true | false
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
        "digraph graphname {\nn1 [label=\"UnionType 1\"];\nn1 -> n2;\nn2 [label=\"SingletonType string: hi\"];\nn1 -> n3;\nn3 [label=\"SingletonType string: \\\"hello\\\"\"];\nn1 -> n4;\nn4 [label=\"SingletonType boolean: true\"];\nn1 -> n5;\nn5 [label=\"SingletonType boolean: false\"];\n}",
        to_dot(fixture.require_type_string(&String::from("x")), &opts)
    );
}
