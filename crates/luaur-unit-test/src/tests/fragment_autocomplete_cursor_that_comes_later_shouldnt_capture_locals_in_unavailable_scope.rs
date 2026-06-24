//! Generated skeleton item.
//! Node: `cxx:Test:Luau.UnitTest:tests/FragmentAutocomplete.test.cpp:1036:fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope`
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
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope

#[cfg(test)]
#[test]
fn fragment_autocomplete_cursor_that_comes_later_shouldnt_capture_locals_in_unavailable_scope() {
    use crate::records::fragment_autocomplete_fixture::FragmentAutocompleteFixture;
    use alloc::string::String;
    use core::ffi::CStr;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::position::Position;
    use luaur_ast::rtti::ast_node_as;

    let mut fixture = FragmentAutocompleteFixture::default();
    let result = fixture.base.run_autocomplete_visitor(
        &String::from(
            r#"
local x = 4
local y = 5
if x == 4 then
    local e = y
end
local z = x + x
if y == 5 then
    local q = x + y + z
end
"#,
        ),
        &Position {
            line: 8,
            column: 23,
        },
    );

    assert_eq!(6, result.ancestry.len());
    assert_eq!(3, result.localStack.len());
    assert_eq!(result.localMap.size(), result.localStack.len());
    assert!(!result.nearestStatement.is_null());
    let last = *result.localStack.last().unwrap();
    assert_eq!(
        "z",
        unsafe { CStr::from_ptr((*last).name.value) }
            .to_str()
            .unwrap()
    );

    let local = unsafe { ast_node_as::<AstStatLocal>(result.nearestStatement as *mut AstNode) };
    assert!(!local.is_null());
    assert_eq!(1, unsafe { (*local).vars.size });
    let var = unsafe { *(*local).vars.data };
    assert_eq!(
        "q",
        unsafe { CStr::from_ptr((*var).name.value) }
            .to_str()
            .unwrap()
    );
}
