use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::lu_tag_iterator::LU_TAG_ITERATOR;

pub fn translate_inst_for_g_prep_inext(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let insn = unsafe { *pc };
    let ra = LUAU_INSN_A(insn) as u8;

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(insn)) as u32);
    let fallback = build.fallback_block(pcpos as u32);
    let finish = build.block(IrBlockKind::Internal);

    build.check_safe_env(pcpos);

    let vm_reg_ra_plus_1 = build.vm_reg(ra + 1);
    let tag_b = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra_plus_1);
    let const_tag_table = build.const_tag(lua_Type::LUA_TTABLE as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_b, const_tag_table, fallback);

    let vm_reg_ra_plus_2 = build.vm_reg(ra + 2);
    let tag_c = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra_plus_2);
    let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tag_c, const_tag_number, fallback);

    let num_c = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, vm_reg_ra_plus_2);
    let const_double_zero = build.const_double(0.0);
    let cond_not_equal = build.cond(IrCondition::NotEqual);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_CMP_NUM,
        num_c,
        const_double_zero,
        cond_not_equal,
        fallback,
        finish,
    );

    build.begin_block(finish);

    let vm_reg_ra = build.vm_reg(ra);
    let const_tag_nil = build.const_tag(lua_Type::LUA_TNIL as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra, const_tag_nil);

    let vm_reg_ra_plus_2 = build.vm_reg(ra + 2);
    let const_int_zero = build.const_int(0);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, vm_reg_ra_plus_2, const_int_zero);

    let const_int_lu_tag_iterator = build.const_int(LU_TAG_ITERATOR);
    build.inst_ir_cmd_ir_op_ir_op(
        IrCmd::STORE_EXTRA,
        vm_reg_ra_plus_2,
        const_int_lu_tag_iterator,
    );

    let const_tag_lightuserdata = build.const_tag(lua_Type::LUA_TLIGHTUSERDATA as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_ra_plus_2, const_tag_lightuserdata);

    build.inst_ir_cmd_ir_op(IrCmd::JUMP, target);

    build.begin_block(fallback);
    let const_uint_pcpos = build.const_uint(pcpos as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(
        IrCmd::FORGPREP_XNEXT_FALLBACK,
        const_uint_pcpos,
        vm_reg_ra,
        target,
    );
}
