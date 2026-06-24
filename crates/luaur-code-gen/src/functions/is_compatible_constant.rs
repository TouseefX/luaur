use crate::enums::ir_const_kind::IrConstKind;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

pub fn is_compatible_constant(build: &mut IrBuilder, arg: IrOp, expected: IrConstKind) -> bool {
    arg.kind() != IrOpKind::Constant || build.function.const_op(arg).kind == expected
}
