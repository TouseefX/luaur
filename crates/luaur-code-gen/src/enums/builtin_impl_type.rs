#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinImplType {
    None,
    UsesFallback,
    Full,
}

impl Default for BuiltinImplType {
    fn default() -> Self {
        Self::None
    }
}

#[allow(non_upper_case_globals)]
impl BuiltinImplType {
    pub const None: Self = Self::None;
    pub const UsesFallback: Self = Self::UsesFallback;
    pub const Full: Self = Self::Full;
}
