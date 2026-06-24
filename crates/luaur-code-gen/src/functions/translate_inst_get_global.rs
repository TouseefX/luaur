use crate::enums::ir_cmd::IrCmd;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_vm::type_aliases::lua_node::LuaNode;

pub fn translate_inst_get_global(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let insn = unsafe { *pc };
    let ra = LUAU_INSN_A(insn) as u8;
    let aux = unsafe { *pc.add(1) };

    let fallback = build.fallback_block(pcpos as u32);

    let env = build.inst_ir_cmd(IrCmd::LOAD_ENV);
    let pcpos_op = build.const_uint(pcpos as u32);
    let aux_op = build.vm_const(aux);
    let addr_slot_el =
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, env, pcpos_op, aux_op);

    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_SLOT_MATCH, addr_slot_el, aux_op, fallback);

    let offset_val = build.const_int(core::mem::offset_of!(LuaNode, val) as i32);
    let tvn = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, addr_slot_el, offset_val);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, tvn);

    let next = build.block_at_inst((pcpos + 2) as u32);
    let mut scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::FALLBACK_GETGLOBAL, pcpos_op, reg_ra, aux_op);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
