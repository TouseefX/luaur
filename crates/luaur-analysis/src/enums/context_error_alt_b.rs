#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Context {
    Binding,
    Type,
}

impl Context {
    pub const Binding: Self = Self::Binding;
    pub const Type: Self = Self::Type;
}
