#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RequireSuggestion {
    pub label: alloc::string::String,
    pub full_path: alloc::string::String,
    pub tags: alloc::vec::Vec<alloc::string::String>,
}
