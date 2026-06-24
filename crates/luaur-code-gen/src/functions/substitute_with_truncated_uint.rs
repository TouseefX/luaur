use crate::enums::ir_cmd::IrCmd;
use crate::functions::produces_dirty_high_register_bits::produces_dirty_high_register_bits;
use crate::functions::replace_ir_utils_alt_b::replace_ir_function_ir_block_u32_ir_inst;
use crate::functions::substitute::substitute;
use crate::records::ir_block::IrBlock;
use crate::records::ir_function::IrFunction;
use crate::records::ir_inst::IrInst;
use crate::records::ir_op::IrOp;
use crate::type_aliases::ir_ops::IrOps;

pub fn substitute_with_truncated_uint(
    function: &mut IrFunction,
    block: &mut IrBlock,
    inst: &mut IrInst,
    op: IrOp,
) {
    let src_of_src: *mut IrInst = function.as_inst_op(op);
    if !src_of_src.is_null() && produces_dirty_high_register_bits(unsafe { (*src_of_src).cmd }) {
        let inst_index = function.get_inst_index(inst);
        let mut ops = IrOps::new();
        ops.push(op);
        let replacement = IrInst {
            cmd: IrCmd::TRUNCATE_UINT,
            ops,
            ..Default::default()
        };
        replace_ir_function_ir_block_u32_ir_inst(function, block, inst_index, replacement);
    } else {
        substitute(function, inst, op);
    }
}
