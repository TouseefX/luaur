#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
pub enum PackField {
    /// What arguments this type accepts.
    Arguments,
    /// What this type returns when called.
    Returns,
    /// The tail of a type pack.
    Tail,
}

impl PackField {
    pub const Arguments: Self = Self::Arguments;
    pub const Returns: Self = Self::Returns;
    pub const Tail: Self = Self::Tail;
}
