#[derive(Debug, Clone, Default, Hash, Eq, PartialEq)]
pub struct AliasInfo {
    pub value: alloc::string::String,
    pub config_location: alloc::string::String,
    pub original_case: alloc::string::String,
}
