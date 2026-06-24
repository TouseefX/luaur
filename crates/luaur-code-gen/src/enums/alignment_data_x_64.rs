#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[allow(non_camel_case_types)]
#[repr(i32)]
pub enum AlignmentDataX64 {
    Nop,
    Int3,
    Ud2,
}

impl Default for AlignmentDataX64 {
    fn default() -> Self {
        Self::Nop
    }
}

impl AlignmentDataX64 {
    pub const Nop: Self = Self::Nop;
    pub const Int3: Self = Self::Int3;
    pub const Ud2: Self = Self::Ud2;
}
