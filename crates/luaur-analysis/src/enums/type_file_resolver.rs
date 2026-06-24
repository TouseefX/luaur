#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Type {
    None,
    Module,
    Script,
}

impl Type {
    pub const None: Self = Self::None;
    pub const Module: Self = Self::Module;
    pub const Script: Self = Self::Script;
}
