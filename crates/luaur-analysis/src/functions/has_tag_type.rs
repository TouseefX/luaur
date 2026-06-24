extern crate alloc;

use crate::type_aliases::tags::Tags;

#[allow(non_snake_case)]
pub fn has_tag(tags: &Tags, tag_name: &str) -> bool {
    tags.iter().any(|t| t == tag_name)
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use has_tag as has_tag_tags_string;
