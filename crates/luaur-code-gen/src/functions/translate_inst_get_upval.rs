use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;

pub fn translate_inst_get_upval(build: &mut IrBuilder, pc: *const Instruction, _pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let up = LUAU_INSN_B(unsafe { *pc }) as u8;

    let vm_upvalue = build.vm_upvalue(up);
    let value = build.inst_ir_cmd_ir_op(IrCmd::GET_UPVALUE, vm_upvalue);

    let vm_reg = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, vm_reg, value);
}
