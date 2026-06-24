use crate::functions::translate_inst_load_constant::translate_inst_load_constant;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;

pub fn translate_inst_load_k(build: &mut IrBuilder, pc: *const Instruction) {
    let insn = unsafe { *pc };
    translate_inst_load_constant(build, LUAU_INSN_A(insn) as i32, LUAU_INSN_D(insn));
}
