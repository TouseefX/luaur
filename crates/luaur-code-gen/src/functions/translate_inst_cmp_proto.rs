use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_cmp_proto(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let aux = unsafe { *pc.add(1) };

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(unsafe { *pc }) as i32) as u32);
    let next = build.block_at_inst((pcpos + 2) as u32);
    let check_fun_id = build.block(IrBlockKind::Internal);

    let vm_reg_ra = build.vm_reg(ra);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);
    let const_tag_function = build.const_tag(lua_Type::LUA_TFUNCTION as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_EQ_TAG,
        ta,
        const_tag_function,
        check_fun_id,
        target,
    );

    build.begin_block(check_fun_id);
    let vm_reg_ra = build.vm_reg(ra);
    let ccl = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, vm_reg_ra);
    let vb = build.const_uint(aux as u32);

    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(IrCmd::JUMP_CMP_PROTOID, ccl, vb, next, target);

    // Fallthrough in original bytecode is implicit, so we start next internal block here
    if build.is_internal_block(next) {
        build.begin_block(next);
    }
}
