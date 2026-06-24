use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;

pub fn translate_inst_jump(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let target = pcpos + 1 + LUAU_INSN_D(unsafe { *pc });
    let block = build.block_at_inst(target as u32);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
}
