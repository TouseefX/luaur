use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_const::IrConst;
use crate::records::ir_function::IrFunction;
use crate::records::ir_op::IrOp;

impl IrFunction {
    pub fn as_tag_op(&mut self, op: IrOp) -> Option<u8> {
        if op.kind() != IrOpKind::Constant {
            return None;
        }

        let value: IrConst = IrFunction::const_op(self, op);

        if value.kind != IrConstKind::Tag {
            return None;
        }

        unsafe { Some(value.value.value_tag) }
    }
}
