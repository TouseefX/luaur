#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct AliasOptions {
    pub config_location: Option<alloc::string::String>,
    pub overwrite_aliases: bool,
}
