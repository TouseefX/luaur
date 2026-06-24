use crate::enums::autocomplete_entry_kind::AutocompleteEntryKind;
use crate::records::autocomplete_entry::AutocompleteEntry;
use crate::records::require_suggestion::RequireSuggestion;
use crate::type_aliases::autocomplete_entry_map::AutocompleteEntryMap;
use crate::type_aliases::require_suggestions::RequireSuggestions;
use alloc::collections::BTreeMap;

pub fn convert_require_suggestions_to_autocomplete_entry_map(
    suggestions: Option<RequireSuggestions>,
) -> Option<AutocompleteEntryMap> {
    let suggestions = suggestions?;

    let mut result = BTreeMap::new();
    for mut suggestion in suggestions {
        let mut entry = AutocompleteEntry::default();
        entry.kind = AutocompleteEntryKind::RequirePath;
        entry.insert_text = Some(suggestion.full_path);
        entry.tags = suggestion.tags;

        result.insert(suggestion.label, entry);
    }

    Some(result)
}
