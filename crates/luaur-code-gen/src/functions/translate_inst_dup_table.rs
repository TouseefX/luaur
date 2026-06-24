use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;

#[allow(non_snake_case)]
pub fn translate_inst_dup_table(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let k = LUAU_INSN_D(unsafe { *pc }) as u32;

    let saved_pc = build.const_uint(pcpos as u32 + 1);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, saved_pc);

    let vm_k = build.vm_const(k);
    let table = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, vm_k);

    let va = build.inst_ir_cmd_ir_op(IrCmd::DUP_TABLE, table);

    let ra_reg = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, ra_reg, va);

    let tag = build.const_tag(lua_Type::LUA_TTABLE as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, tag);

    build.inst_ir_cmd(IrCmd::CHECK_GC);
}
