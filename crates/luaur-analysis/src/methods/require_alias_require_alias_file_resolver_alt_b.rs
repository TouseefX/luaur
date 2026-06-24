use crate::records::require_alias::RequireAlias;

impl RequireAlias {
    pub fn require_alias_string_vector_string(
        alias: alloc::string::String,
        tags: alloc::vec::Vec<alloc::string::String>,
    ) -> Self {
        Self { alias, tags }
    }
}
