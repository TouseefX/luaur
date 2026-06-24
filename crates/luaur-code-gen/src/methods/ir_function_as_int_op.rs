use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

impl IrFunction {
    pub fn as_int_op(&mut self, op: IrOp) -> Option<i32> {
        if op.kind() != IrOpKind::Constant {
            return None;
        }

        let value = self.const_op(op);

        if value.kind != IrConstKind::Int {
            return None;
        }

        unsafe { Some(value.value.value_int) }
    }
}
