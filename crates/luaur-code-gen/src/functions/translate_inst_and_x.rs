use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;

pub fn translate_inst_and_x(build: &mut IrBuilder, pc: *const Instruction, _pcpos: i32, c: IrOp) {
    let insn = unsafe { *pc };
    let ra = LUAU_INSN_A(insn) as u8;
    let rb = LUAU_INSN_B(insn) as u8;

    // "b and c" -> "truthy(b) ? c : b"
    let rb_reg = build.vm_reg(rb);
    let lhs = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, rb_reg);
    let rhs = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, c);

    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::SELECT_IF_TRUTHY, lhs, rhs, lhs);

    let ra_reg = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, ra_reg, result);
}
