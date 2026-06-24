extern crate alloc;

use crate::records::require_alias::RequireAlias;
use crate::records::require_suggestion::RequireSuggestion;
use crate::type_aliases::require_suggestions::RequireSuggestions;
use alloc::string::ToString;
use alloc::vec::Vec;

pub(crate) fn make_suggestions_from_aliases(aliases: Vec<RequireAlias>) -> RequireSuggestions {
    let mut result = RequireSuggestions::with_capacity(aliases.len());
    for mut alias in aliases {
        let label = "@".to_string() + &alias.alias;
        let suggestion = RequireSuggestion {
            label: label.clone(),
            full_path: label,
            tags: core::mem::take(&mut alias.tags),
        };
        result.push(suggestion);
    }
    result
}
