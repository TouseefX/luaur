use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_c::LUAU_INSN_C;

pub fn translate_inst_load_b(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let pc_val = unsafe { *pc };
    let ra = LUAU_INSN_A(pc_val) as u8;
    let b = LUAU_INSN_B(pc_val);
    let c = LUAU_INSN_C(pc_val);

    let ra_op = build.vm_reg(ra);
    let b_op = build.const_int(b as i32);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, ra_op, b_op);

    let tag_op = build.const_tag(1); // LUA_TBOOLEAN = 1
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_op, tag_op);

    if c != 0 {
        let target = pcpos + 1 + (c as i32);
        let block = build.block_at_inst(target as u32);
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, block);
    }
}
