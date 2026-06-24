extern crate alloc;

use crate::enums::autocomplete_context::AutocompleteContext;
use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::records::autocomplete_result::AutocompleteResult;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use alloc::string::ToString;
use alloc::vec::Vec;

pub fn autocomplete_while_loop_keywords(
    ancestry: Vec<*mut luaur_ast::records::ast_node::AstNode>,
) -> AutocompleteResult {
    let mut entry_map: AutocompleteEntryMap = AutocompleteEntryMap::new();

    entry_map.insert(
        "do".to_string(),
        crate::records::autocomplete_entry::AutocompleteEntry {
            kind: AutocompleteEntryKind::Keyword,
            ..crate::records::autocomplete_entry::AutocompleteEntry::default()
        },
    );
    entry_map.insert(
        "and".to_string(),
        crate::records::autocomplete_entry::AutocompleteEntry {
            kind: AutocompleteEntryKind::Keyword,
            ..crate::records::autocomplete_entry::AutocompleteEntry::default()
        },
    );
    entry_map.insert(
        "or".to_string(),
        crate::records::autocomplete_entry::AutocompleteEntry {
            kind: AutocompleteEntryKind::Keyword,
            ..crate::records::autocomplete_entry::AutocompleteEntry::default()
        },
    );

    AutocompleteResult::autocomplete_result_autocomplete_entry_map_vector_ast_node_autocomplete_context(
        entry_map,
        ancestry,
        AutocompleteContext::Keyword,
    )
}
