use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;

pub fn translate_inst_jump_back(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let interrupt_op = build.const_uint(pcpos as u32);
    build.inst_ir_cmd_ir_op(IrCmd::INTERRUPT, interrupt_op);

    let d = unsafe { LUAU_INSN_D(*pc) };
    let target_block = build.block_at_inst((pcpos + 1 + d) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, target_block);
}
