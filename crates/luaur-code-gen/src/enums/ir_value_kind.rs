#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrValueKind {
    Unknown, // Used by SUBSTITUTE, argument has to be checked to get type
    None,
    Tag,
    Int,
    Int64,
    Pointer,
    Float,
    Double,
    Tvalue,

    Count,
}

#[allow(non_upper_case_globals)]
impl IrValueKind {
    pub const Unknown: Self = Self::Unknown;
    pub const None: Self = Self::None;
    pub const Tag: Self = Self::Tag;
    pub const Int: Self = Self::Int;
    pub const Int64: Self = Self::Int64;
    pub const Pointer: Self = Self::Pointer;
    pub const Float: Self = Self::Float;
    pub const Double: Self = Self::Double;
    pub const Tvalue: Self = Self::Tvalue;

    pub const Count: Self = Self::Count;
}

/// `static constexpr unsigned kValueDwordSize[] = {0, 0, 1, 1, 2, 2, 1, 2, 4};`
/// (IrRegAllocX64.cpp:20) — indexed by `IrValueKind`, one entry per kind.
pub const K_VALUE_DWORD_SIZE: [u32; 9] = [0, 0, 1, 1, 2, 2, 1, 2, 4];
