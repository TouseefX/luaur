#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RequireAlias {
    pub(crate) alias: alloc::string::String,
    pub(crate) tags: alloc::vec::Vec<alloc::string::String>,
}
