use crate::functions::translate_inst_binary_numeric::translate_inst_binary_numeric;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_c::LUAU_INSN_C;
use luaur_vm::type_aliases::tms::TMS;

pub fn translate_inst_binary_rk(
    build: &mut IrBuilder,
    pc: *const Instruction,
    pcpos: i32,
    tm: TMS,
) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let rb = LUAU_INSN_B(unsafe { *pc }) as u8;
    let rc = LUAU_INSN_C(unsafe { *pc }) as u8;

    let opb = build.vm_const(rb as u32);
    let opc = build.vm_reg(rc);

    translate_inst_binary_numeric(build, ra as i32, -1, rc as i32, opb, opc, pcpos, tm);
}
