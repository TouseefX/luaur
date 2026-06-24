extern crate alloc;

use crate::functions::make_suggestions_from_aliases::make_suggestions_from_aliases;
use crate::records::require_node::RequireNode;
use crate::records::require_suggestion::RequireSuggestion;
use crate::type_aliases::require_suggestions::RequireSuggestions;
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;

pub(crate) fn make_suggestions_for_first_component(
    node: Box<dyn RequireNode>,
) -> RequireSuggestions {
    let mut result = make_suggestions_from_aliases(node.get_available_aliases());

    result.push(RequireSuggestion {
        label: "./".to_string(),
        full_path: "./".to_string(),
        tags: Vec::new(),
    });

    result.push(RequireSuggestion {
        label: "../".to_string(),
        full_path: "../".to_string(),
        tags: Vec::new(),
    });

    result
}
