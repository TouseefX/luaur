#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IrBlockKind {
    Bytecode,
    Fallback,
    Internal,
    Linearized,
    ExitSync,
    Dead,
}

#[allow(non_upper_case_globals)]
impl IrBlockKind {
    pub const Bytecode: Self = Self::Bytecode;
    pub const Fallback: Self = Self::Fallback;
    pub const Internal: Self = Self::Internal;
    pub const Linearized: Self = Self::Linearized;
    pub const ExitSync: Self = Self::ExitSync;
    pub const Dead: Self = Self::Dead;
}
