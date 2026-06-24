use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::position::Position;
use luaur_ast::rtti::ast_node_is;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn autocomplete_keywords(
    ancestry: &alloc::vec::Vec<*mut AstNode>,
    _position: Position,
    result: &mut AutocompleteEntryMap,
) {
    LUAU_ASSERT!(!ancestry.is_empty());

    let node = unsafe { *ancestry.last().unwrap() };

    let is_expr_function = unsafe { ast_node_is::<AstExprFunction>(&*(node as *mut AstNode)) };
    let is_expr = unsafe { !(*node).as_expr().is_null() };

    if !is_expr_function && is_expr {
        result.insert(
            "and".to_string(),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                ..Default::default()
            },
        );
        result.insert(
            "or".to_string(),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                ..Default::default()
            },
        );
        result.insert(
            "not".to_string(),
            AutocompleteEntry {
                kind: AutocompleteEntryKind::Keyword,
                ..Default::default()
            },
        );
    }
}
