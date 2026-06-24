//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:388:to_dot_class`
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
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - translates_to -> rust_item to_dot_class

#[cfg(test)]
#[test]
fn to_dot_class() {
    use crate::records::to_dot_class_fixture::ToDotClassFixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;

    let mut fixture = ToDotClassFixture::default();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
local a: ChildClass
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
        "digraph graphname {\nn1 [label=\"ExternType ChildClass\"];\nn1 -> n2 [label=\"ChildField\"];\nn2 [label=\"string\"];\nn1 -> n3 [label=\"[parent]\"];\nn3 [label=\"ExternType BaseClass\"];\nn3 -> n4 [label=\"BaseField\"];\nn4 [label=\"number\"];\nn3 -> n5 [label=\"[metatable]\"];\nn5 [label=\"TableType 5\"];\n}",
        to_dot(fixture.base.require_type_string(&String::from("a")), &opts)
    );
}
