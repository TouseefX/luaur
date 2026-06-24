use crate::enums::ir_const_kind::IrConstKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_const::IrConst;
use crate::records::ir_op::IrOp;

impl IrBuilder {
    pub fn const_int(&mut self, value: i32) -> IrOp {
        let constant = IrConst {
            kind: IrConstKind::Int,
            value: unsafe {
                core::mem::transmute::<u64, crate::records::ir_const::IrConstValue>(value as u64)
            },
        };

        self.const_any(constant, value as u64)
    }
}
