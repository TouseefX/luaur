use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

impl IrFunction {
    pub fn as_uint_op(&mut self, op: IrOp) -> Option<u32> {
        if op.kind() != IrOpKind::Constant {
            return None;
        }

        let value = self.const_op(op);

        if value.kind != IrConstKind::Uint {
            return None;
        }

        unsafe { Some(value.value.value_uint) }
    }
}
