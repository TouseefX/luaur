use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;

pub fn builtin_load_double(build: &mut IrBuilder, arg: IrOp) -> IrOp {
    if arg.kind() == IrOpKind::Constant {
        return arg;
    }

    build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, arg)
}
