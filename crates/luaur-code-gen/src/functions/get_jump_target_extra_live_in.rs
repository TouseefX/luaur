use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::is_non_terminating_jump::is_non_terminating_jump;
use crate::functions::require_variadic_sequence::require_variadic_sequence;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::op_a::op_a;
use crate::records::ir_block::IrBlock;
use crate::records::ir_inst::IrInst;
use crate::records::ir_to_string_context::IrToStringContext;
use crate::records::register_set::RegisterSet;

pub fn get_jump_target_extra_live_in(
    ctx: &mut IrToStringContext,
    _block: &IrBlock,
    block_idx: u32,
    inst: &mut IrInst,
) -> RegisterSet {
    let mut extra_rs = RegisterSet::default();

    if block_idx as usize >= ctx.cfg.r#in.len() {
        return extra_rs;
    }

    let def_rs = ctx.cfg.r#in[block_idx as usize];

    // Find first block argument, for guard instructions (isNonTerminatingJump), that's the first and only one
    CODEGEN_ASSERT!(is_non_terminating_jump(inst.cmd));
    let mut op = op_a(inst);

    for i in 1..inst.ops.size() as usize {
        if inst.ops.as_slice()[i].kind() == IrOpKind::Block {
            op = inst.ops.as_slice()[i];
            break;
        }
    }

    if op.kind() == IrOpKind::Block && (op.index() as usize) < ctx.cfg.r#in.len() {
        let in_rs = ctx.cfg.r#in[op.index() as usize];

        for w in 0..4 {
            extra_rs.regs[w] = in_rs.regs[w] & !def_rs.regs[w];
        }

        if in_rs.vararg_seq {
            require_variadic_sequence(&mut extra_rs, &def_rs, in_rs.vararg_start);
        }
    }

    extra_rs
}
