use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::get_jump_target::get_jump_target;
use crate::functions::get_op_length::get_op_length;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;

use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;

use luaur_vm::enums::lua_type::lua_Type;

use crate::macros::codegen_assert::CODEGEN_ASSERT;

pub fn translate_inst_for_n_prep(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;

    let op = LuauOpcode::from(LUAU_INSN_OP(unsafe { *pc }) as u8);
    let loop_start = build.block_at_inst((pcpos + get_op_length(op)) as u32);
    let loop_exit = build.block_at_inst(get_jump_target(unsafe { *pc }, pcpos as u32) as u32);

    CODEGEN_ASSERT!(!build.numeric_loop_stack.is_empty());
    let step_k = build.numeric_loop_stack.last().unwrap().step;

    let reg_limit = build.vm_reg(ra);
    let tag_limit = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_limit);
    let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_limit, number_tag, exit);

    let reg_idx = build.vm_reg(ra + 2);
    let tag_idx = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_idx);
    let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    let exit = build.vm_exit(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_idx, number_tag, exit);

    let reg_limit = build.vm_reg(ra);
    let limit = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_limit);
    let reg_idx = build.vm_reg(ra + 2);
    let idx = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_idx);

    if step_k.kind() == crate::enums::ir_op_kind::IrOpKind::Undef {
        let reg_step = build.vm_reg(ra + 1);
        let tag_step = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_step);
        let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_step, number_tag, exit);

        let reg_step = build.vm_reg(ra + 1);
        let step = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_step);

        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::JUMP_FORN_LOOP_COND,
            idx,
            limit,
            step,
            loop_start,
            loop_exit,
        );
    } else {
        let step_n = build.function.double_op(step_k);
        let cond = build.cond(IrCondition::NotLessEqual);

        if step_n > 0.0 {
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
                IrCmd::JUMP_CMP_NUM,
                idx,
                limit,
                cond,
                loop_exit,
                loop_start,
            );
        } else {
            build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
                IrCmd::JUMP_CMP_NUM,
                limit,
                idx,
                cond,
                loop_exit,
                loop_start,
            );
        }
    }

    if build.is_internal_block(loop_start) {
        build.begin_block(loop_start);
    }

    build.interrupt_requested = true;
}
