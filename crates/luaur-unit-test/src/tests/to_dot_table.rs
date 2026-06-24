//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/ToDot.test.cpp:235:to_dot_table`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ToDotOptions (Analysis/include/Luau/ToDot.h)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record VariadicTypePack (Analysis/include/Luau/TypePack.h)
//!   - type_ref -> record TypePack (Analysis/include/Luau/TypePack.h)
//!   - translates_to -> rust_item to_dot_table

#[cfg(test)]
#[test]
fn to_dot_table() {
    use crate::records::fixture::Fixture;
    use luaur_analysis::functions::to_dot_to_dot::to_dot;
    use luaur_analysis::records::to_dot_options::ToDotOptions;

    let mut fixture = Fixture::default();
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type A<T, U...> = { x: T, y: (U...) -> (), [string]: any }
local a: A<number, ...string>
"#,
        ),
        None,
    );
    assert!(result.errors.is_empty(), "{:?}", result.errors);

    let a_ty = fixture.require_type_string(&String::from("a"));
    let opts = ToDotOptions {
        show_pointers: false,
        duplicate_primitives: true,
    };

    assert_eq!(
        "digraph graphname {\nn1 [label=\"TableType A\"];\nn1 -> n2 [label=\"x\"];\nn2 [label=\"number\"];\nn1 -> n3 [label=\"y\"];\nn3 [label=\"FunctionType 3\"];\nn3 -> n4 [label=\"arg\"];\nn4 [label=\"VariadicTypePack 4\"];\nn4 -> n5;\nn5 [label=\"string\"];\nn3 -> n6 [label=\"ret\"];\nn6 [label=\"TypePack 6\"];\nn1 -> n7 [label=\"[index]\"];\nn7 [label=\"string\"];\nn1 -> n8 [label=\"[value]\"];\nn8 [label=\"any\"];\nn1 -> n9 [label=\"typeParam\"];\nn9 [label=\"number\"];\nn1 -> n4 [label=\"typePackParam\"];\n}",
        to_dot(a_ty, &opts)
    );

    let _ = luaur_analysis::functions::to_dot_to_dot_alt_c::to_dot(a_ty);
}
