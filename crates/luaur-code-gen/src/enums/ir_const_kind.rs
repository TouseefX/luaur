#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrConstKind {
    Int,
    Int64,
    Uint,
    Double,
    Tag,
    Import,
}

#[allow(non_upper_case_globals)]
impl IrConstKind {
    pub const Int: Self = Self::Int;
    pub const Int64: Self = Self::Int64;
    pub const Uint: Self = Self::Uint;
    pub const Double: Self = Self::Double;
    pub const Tag: Self = Self::Tag;
    pub const Import: Self = Self::Import;
}
