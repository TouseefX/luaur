use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_not(build: &mut IrBuilder, pc: *const Instruction) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

    let rb_reg = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, rb_reg);
    let rb_reg = build.vm_reg(rb);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_INT, rb_reg);

    let va = build.inst_ir_cmd_ir_op_ir_op(IrCmd::NOT_ANY, tb, vb);

    let ra_reg = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, ra_reg, va);
    let ra_reg = build.vm_reg(ra);
    let boolean_tag = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_reg, boolean_tag);
}
