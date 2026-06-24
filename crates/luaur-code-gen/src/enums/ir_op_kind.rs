#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum IrOpKind {
    None,
    Undef,
    Constant,
    Condition,
    Inst,
    Block,
    VmReg,
    VmConst,
    VmUpvalue,
    VmExit,
}

impl Default for IrOpKind {
    fn default() -> Self {
        Self::None
    }
}

#[allow(non_upper_case_globals)]
impl IrOpKind {
    pub const None: Self = Self::None;
    pub const Undef: Self = Self::Undef;
    pub const Constant: Self = Self::Constant;
    pub const Condition: Self = Self::Condition;
    pub const Inst: Self = Self::Inst;
    pub const Block: Self = Self::Block;
    pub const VmReg: Self = Self::VmReg;
    pub const VmConst: Self = Self::VmConst;
    pub const VmUpvalue: Self = Self::VmUpvalue;
    pub const VmExit: Self = Self::VmExit;
}
