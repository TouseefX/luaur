#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum Kind {
    Type,
    Pack,
}

impl Kind {
    pub const Type: Self = Self::Type;
    pub const Pack: Self = Self::Pack;
}
