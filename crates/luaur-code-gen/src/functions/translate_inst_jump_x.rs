use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_e::LUAU_INSN_E;

pub fn translate_inst_jump_x(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let interrupt_op = build.const_uint(pcpos as u32);
    build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_op);

    let jump_target = pcpos + 1 + LUAU_INSN_E(unsafe { *pc });
    let jump_block = build.block_at_inst(jump_target as u32);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, jump_block);
}
