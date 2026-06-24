use crate::functions::get_loop_step_k::get_loop_step_k;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;

pub fn before_inst_for_n_prep(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as i32;
    let step_k = get_loop_step_k(build, ra);
    build
        .numeric_loop_stack
        .push(crate::records::ir_builder::LoopInfo {
            step: step_k,
            startpc: pcpos + 1,
        });
}
