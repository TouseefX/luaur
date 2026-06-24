use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_op_kind::IrOpKind;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;

use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;

use crate::functions::get_jump_target::get_jump_target;
use crate::functions::get_op_length::get_op_length;
use crate::macros::codegen_assert::CODEGEN_ASSERT;

pub fn translate_inst_for_n_loop(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;

    let repeat_jump_target = get_jump_target(unsafe { *pc }, pcpos as u32);
    let loop_repeat = build.block_at_inst(repeat_jump_target as u32);
    let op = LuauOpcode::from(LUAU_INSN_OP(unsafe { *pc }) as u8);
    let op_length = get_op_length(op);
    let loop_exit = build.block_at_inst((pcpos + op_length) as u32);

    CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
    let loop_info = build.numeric_loop_stack.last().copied().unwrap();
    let step_k = loop_info.step;

    if repeat_jump_target != loop_info.startpc {
        let pcpos_op = build.const_uint(pcpos as u32);
        build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, pcpos_op);
    }

    let reg_limit = build.vm_reg(ra);
    let limit = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_limit);
    let step = if step_k.kind() == IrOpKind::Undef {
        let reg_step = build.vm_reg(ra + 1);
        build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_step)
    } else {
        step_k
    };

    let reg_idx = build.vm_reg(ra + 2);
    let mut idx = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_idx);
    idx = build.inst_ir_cmd_ir_op_ir_op(IrCmd::ADD_NUM, idx, step);
    let reg_idx = build.vm_reg(ra + 2);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, reg_idx, idx);

    if step_k.kind() == IrOpKind::Undef {
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::JUMP_FORN_LOOP_COND,
            idx,
            limit,
            step,
            loop_repeat,
            loop_exit,
        );
    } else {
        let step_n = build.function.double_op(step_k);

        let reg_step = build.vm_reg(ra + 1);
        let one = build.const_int(1);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::MARK_USED, reg_step, one);

        let cond = if step_n > 0.0 {
            IrCondition::LessEqual
        } else {
            IrCondition::LessEqual
        };
        let cond_op = build.cond(cond);

        if step_n > 0.0 {
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
                IrCmd::JUMP_CMP_NUM,
                idx,
                limit,
                cond_op,
                loop_repeat,
                loop_exit,
            );
        } else {
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
                IrCmd::JUMP_CMP_NUM,
                limit,
                idx,
                cond_op,
                loop_repeat,
                loop_exit,
            );
        }
    }

    if build.is_internal_block(loop_exit) {
        build.begin_block(loop_exit);
    }
}
