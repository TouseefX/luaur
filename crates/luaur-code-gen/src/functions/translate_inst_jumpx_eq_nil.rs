use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;

use crate::enums::ir_cmd::IrCmd;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_jumpx_eq_nil(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let not_ = (unsafe { *pc.add(1) } & 0x80000000) != 0;

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(unsafe { *pc }) as i32) as u32);
    let next = build.block_at_inst((pcpos + 2) as u32);

    let vm_reg_ra = build.vm_reg(ra);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);
    let const_nil_tag = build.const_tag(lua_Type::LUA_TNIL as u8);

    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_EQ_TAG,
        ta,
        const_nil_tag,
        if not_ { next } else { target },
        if not_ { target } else { next },
    );

    // Fallthrough in original bytecode is implicit, so we start next internal block here
    if build.is_internal_block(next) {
        build.begin_block(next);
    }
}
