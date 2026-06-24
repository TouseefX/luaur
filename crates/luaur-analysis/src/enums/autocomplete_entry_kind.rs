#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AutocompleteEntryKind {
    Property,
    Binding,
    Keyword,
    String,
    Type,
    Module,
    GeneratedFunction,
    RequirePath,
    HotComment,
}

impl AutocompleteEntryKind {
    pub const Property: Self = Self::Property;
    pub const Binding: Self = Self::Binding;
    pub const Keyword: Self = Self::Keyword;
    pub const String: Self = Self::String;
    pub const Type: Self = Self::Type;
    pub const Module: Self = Self::Module;
    pub const GeneratedFunction: Self = Self::GeneratedFunction;
    pub const RequirePath: Self = Self::RequirePath;
    pub const HotComment: Self = Self::HotComment;
}
