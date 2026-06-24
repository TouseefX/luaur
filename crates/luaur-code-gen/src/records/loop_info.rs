use crate::records::ir_op::IrOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LoopInfo {
    pub step: IrOp,
    pub startpc: core::ffi::c_int,
}

impl Default for LoopInfo {
    fn default() -> Self {
        Self {
            step: IrOp { kind_and_index: 0 },
            startpc: 0,
        }
    }
}
