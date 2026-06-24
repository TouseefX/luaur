use crate::enums::type_file_resolver::Type;
use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SourceCode {
    pub source: String,
    pub r#type: Type,
}

impl SourceCode {
    pub const None: Type = Type::None;
    pub const Module: Type = Type::Module;
    pub const Script: Type = Type::Script;
}
