use crate::records::require_suggestion::RequireSuggestion;
use crate::type_aliases::require_suggestions::RequireSuggestions;
use luaur_common::functions::escape::escape;

pub fn process_require_suggestions(
    mut suggestions: Option<RequireSuggestions>,
) -> Option<RequireSuggestions> {
    if let Some(ref mut suggestions_vec) = suggestions {
        for suggestion in suggestions_vec {
            suggestion.full_path = escape(&suggestion.full_path, false);
        }
    }

    suggestions
}
