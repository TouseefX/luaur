use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;

pub fn translate_inst_load_nil(build: &mut IrBuilder, pc: *const Instruction) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let vm_reg = build.vm_reg(ra);
    let tag_op = build.const_tag(0);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg, tag_op);
}
