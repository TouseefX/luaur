//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1397:fragment_autocomplete_can_parse_multi_line_fragment_override`
//! Source: `tests/FragmentAutocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/FragmentAutocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/FragmentAutocomplete.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/FragmentAutocomplete.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method FragmentAutocompleteFixtureImpl::checkWithOptions (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method FragmentAutocompleteFixtureImpl::parseFragment (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstStatExpr (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - calls -> method AstArray::rbegin (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprCall (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_can_parse_multi_line_fragment_override

#[cfg(test)]
#[test]
fn fragment_autocomplete_can_parse_multi_line_fragment_override() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::ast_expr_call::AstExprCall;
    use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_expr::AstStatExpr;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture
        .base
        .check_with_options(&String::from("function abc(foo: string) end"));
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let fragment = fixture
        .base
        .parse_fragment(
            &String::from(
                r#"function abc(foo: string) end
abc(
"foo"
)
abc("bar")
"#,
            ),
            &Position { line: 2, column: 5 },
            Some(Position { line: 3, column: 1 }),
        )
        .expect("expected fragment parse result");

    assert_eq!("abc(\n\"foo\"\n)", fragment.fragment_to_parse);
    assert!(!fragment.nearest_statement.is_null());
    assert!(
        !unsafe { ast_node_as::<AstStatExpr>(fragment.nearest_statement as *mut AstNode) }
            .is_null()
    );
    assert!(fragment.ancestry.len() >= 2);

    let back = *fragment.ancestry.last().unwrap();
    assert!(!unsafe { ast_node_as::<AstExprConstantString>(back) }.is_null());
    assert_eq!(Position { line: 2, column: 0 }, unsafe {
        (*back).location.begin
    });
    assert_eq!(Position { line: 2, column: 5 }, unsafe {
        (*back).location.end
    });

    let parent = fragment.ancestry[fragment.ancestry.len() - 2];
    assert!(!unsafe { ast_node_as::<AstExprCall>(parent) }.is_null());
    assert_eq!(Position { line: 1, column: 0 }, unsafe {
        (*parent).location.begin
    });
    assert_eq!(Position { line: 3, column: 1 }, unsafe {
        (*parent).location.end
    });
}
