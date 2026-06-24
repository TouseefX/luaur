use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;

pub fn translate_inst_move(build: &mut IrBuilder, pc: *const Instruction) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

    let load_arg = build.vm_reg(rb);
    let load = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, load_arg);

    let store_arg = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, store_arg, load);
}
