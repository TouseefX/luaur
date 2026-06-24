use crate::functions::translate_inst_load_constant::translate_inst_load_constant;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;

pub fn translate_inst_load_kx(build: &mut IrBuilder, pc: *const Instruction) {
    translate_inst_load_constant(
        build,
        LUAU_INSN_A(unsafe { *pc }) as i32,
        unsafe { *pc.add(1) } as i32,
    );
}
