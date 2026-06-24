//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:136:to_dot_function`
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
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record GenericType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> type_alias BoundTypePack (Analysis/include/Luau/TypePack.h)
//!   - translates_to -> rust_item to_dot_function

#[cfg(test)]
#[test]
fn to_dot_function() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::to_dot_options::ToDotOptions;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
local function f(a, ...: string) return a end
"#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let f_ty = fixture.require_type_string(&String::from("f"));
    assert_eq!("<a>(a, ...string) -> a", to_string_type_id(f_ty));

    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"FunctionType 1\"];\nn1 -> n2 [label=\"arg\"];\nn2 [label=\"TypePack 2\"];\nn2 -> n3;\nn3 [label=\"GenericType 3\"];\nn2 -> n4 [label=\"tail\"];\nn4 [label=\"VariadicTypePack 4\"];\nn4 -> n5;\nn5 [label=\"string\"];\nn1 -> n6 [label=\"ret\"];\nn6 [label=\"BoundTypePack 6\"];\nn6 -> n7;\nn7 [label=\"TypePack 7\"];\nn7 -> n3;\n}",
        to_dot(f_ty, &opts)
    );
}
