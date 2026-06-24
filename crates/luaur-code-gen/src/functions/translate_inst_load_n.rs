use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;

pub fn translate_inst_load_n(build: &mut IrBuilder, pc: *const Instruction) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let vm_reg = build.vm_reg(ra);
    let value = build.const_double(LUAU_INSN_D(unsafe { *pc }) as f64);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, vm_reg, value);
    let tag = build.const_tag(3);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg, tag);
}
