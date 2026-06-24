#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub struct ConfigOptions {
    pub compat: bool,
    pub alias_options: Option<crate::records::alias_options::AliasOptions>,
}

pub use crate::records::alias_options::AliasOptions;
