//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1065:fragment_autocomplete_nearest_enclosing_statement_can_be_non_local`
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
//!   - calls -> method FragmentAutocompleteFixtureImpl::runAutocompleteVisitor (tests/FragmentAutocomplete.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstStatIf (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_nearest_enclosing_statement_can_be_non_local

#[cfg(test)]
#[test]
fn fragment_autocomplete_nearest_enclosing_statement_can_be_non_local() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::string::String;
    use core::ffi::CStr;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_if::AstStatIf;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
        &String::from(
            r#"
local x = 4
local y = 5
if x == 4 then
"#,
        ),
        &Position { line: 3, column: 4 },
    );

    assert_eq!(4, result.ancestry.len());
    assert_eq!(2, result.localStack.len());
    assert_eq!(result.localMap.size(), result.localStack.len());
    assert!(!result.nearestStatement.is_null());
    let last = *result.localStack.last().unwrap();
    assert_eq!(
        "y",
        unsafe { CStr::from_ptr((*last).name.value) }
            .to_str()
            .unwrap()
    );

    let if_stmt = unsafe { ast_node_as::<AstStatIf>(result.nearestStatement as *mut AstNode) };
    assert!(!if_stmt.is_null());
}
