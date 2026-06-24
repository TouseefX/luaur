use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;

use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;

pub fn translate_inst_jump_if(
    build: &mut IrBuilder,
    pc: *const Instruction,
    pcpos: i32,
    not_: bool,
) {
    let ra = LUAU_INSN_A(unsafe { *pc as u32 });

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(unsafe { *pc as u32 })) as u32);
    let next = build.block_at_inst((pcpos + 1) as u32);

    let vm_reg_op = build.vm_reg(ra as u8);

    if not_ {
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JUMP_IF_FALSY, vm_reg_op, target, next);
    } else {
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::JUMP_IF_TRUTHY, vm_reg_op, target, next);
    }

    if build.is_internal_block(next) {
        build.begin_block(next);
    }
}
