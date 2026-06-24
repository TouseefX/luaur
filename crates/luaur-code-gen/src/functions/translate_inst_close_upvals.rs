use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;

pub fn translate_inst_close_upvals(build: &mut IrBuilder, pc: *const Instruction) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let vm_reg = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op(IrCmd::CLOSE_UPVALS, vm_reg);
}
