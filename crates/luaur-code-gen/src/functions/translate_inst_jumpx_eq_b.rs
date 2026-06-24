use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_aux_kb::LUAU_INSN_AUX_KB;
use luaur_common::macros::luau_insn_aux_not::LUAU_INSN_AUX_NOT;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_jumpx_eq_b(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let pc_val = unsafe { *pc };
    let ra = LUAU_INSN_A(pc_val) as u8;
    let aux = unsafe { *pc.add(1) };
    let not_ = LUAU_INSN_AUX_NOT(aux) != 0;

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(pc_val)) as u32);
    let next = build.block_at_inst((pcpos + 2) as u32);
    let check_value = build.block(IrBlockKind::Internal);

    let vm_reg_ra = build.vm_reg(ra);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);

    let const_tag_boolean = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_EQ_TAG,
        ta,
        const_tag_boolean,
        check_value,
        if not_ { target } else { next },
    );

    build.begin_block(check_value);
    let vm_reg_ra = build.vm_reg(ra);
    let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, vm_reg_ra);

    let const_int_kb = build.const_int(LUAU_INSN_AUX_KB(aux) as i32);
    let cond_equal = build.cond(IrCondition::Equal);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_CMP_INT,
        va,
        const_int_kb,
        cond_equal,
        if not_ { next } else { target },
        if not_ { target } else { next },
    );

    if build.is_internal_block(next) {
        build.begin_block(next);
    }
}
