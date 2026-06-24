use crate::records::require_alias::RequireAlias;

impl RequireAlias {
    pub fn require_alias_string(alias: alloc::string::String) -> Self {
        Self {
            alias,
            tags: alloc::vec::Vec::new(),
        }
    }
}
