use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_new_table(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let pc_val = unsafe { *pc };
    let ra = LUAU_INSN_A(pc_val) as u8;
    let b = LUAU_INSN_B(pc_val);
    let aux = unsafe { *pc.add(1) };

    let savedpc_op = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_op);

    let array_size = if b == 0 { 0 } else { 1 << (b - 1) };
    let aux_op = build.const_uint(aux);
    let array_size_op = build.const_uint(array_size);
    let va = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NEW_TABLE, aux_op, array_size_op);

    let ra_op = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, ra_op, va);

    let tag_op = build.const_tag(lua_Type::LUA_TTABLE as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_op, tag_op);

    build.inst_ir_cmd(IrCmd::CHECK_GC);
}
