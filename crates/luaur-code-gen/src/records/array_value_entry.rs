use crate::records::ir_op::IrOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct ArrayValueEntry {
    pub pointer: u32,
    pub offset: IrOp,
    pub value: u32,
}

#[allow(non_upper_case_globals)]
impl ArrayValueEntry {
    pub const pointer: u32 = 0;
    pub const value: u32 = 0;
}

impl Default for ArrayValueEntry {
    fn default() -> Self {
        Self {
            pointer: 0,
            offset: IrOp { kind_and_index: 0 },
            value: 0,
        }
    }
}
