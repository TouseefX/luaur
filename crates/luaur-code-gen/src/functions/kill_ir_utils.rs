use crate::enums::ir_cmd::IrCmd;
use crate::functions::remove_use::remove_use;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;

pub fn kill_ir_function_ir_inst(function: &mut IrFunction, inst: &mut IrInst) {
    CODEGEN_ASSERT!(inst.use_count == 0);

    inst.cmd = IrCmd::NOP;

    let n = inst.ops.size();
    for i in 0..n {
        let op: IrOp = inst.ops.as_slice()[i as usize];
        remove_use(function, op);
    }
    inst.ops.clear();
}
