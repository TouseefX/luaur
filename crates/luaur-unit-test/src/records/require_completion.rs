#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct RequireCompletion {
    pub(crate) label: alloc::string::String,
    pub(crate) insert_text: alloc::string::String,
}
