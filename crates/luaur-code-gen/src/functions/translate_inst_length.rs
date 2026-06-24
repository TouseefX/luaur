use crate::enums::ir_cmd::IrCmd;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_length(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let bc_types = build.function.get_bytecode_types_at(pcpos);

    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

    if is_userdata_bytecode_type(bc_types.a) {
        let savedpc_arg = build.const_uint((pcpos + 1) as u32);
        build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::DO_LEN, reg_ra, reg_rb);
        return;
    }

    let fallback = build.fallback_block(pcpos as u32);

    let vm_reg_rb = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_rb);
    let const_tag_table = build.const_tag(lua_Type::LUA_TTABLE as u8);
    let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
        build.vm_exit(pcpos as u32)
    } else {
        fallback
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, const_tag_table, exit_or_fallback);

    let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, vm_reg_rb);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_NO_METATABLE, vb, fallback);

    let va = build.inst_ir_cmd_ir_op(IrCmd::TABLE_LEN, vb);
    let vai = build.inst_ir_cmd_ir_op(IrCmd::INT_TO_NUM, va);

    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, reg_ra, vai);
    let reg_ra = build.vm_reg(ra);
    let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_ra, number_tag);

    let next = build.block_at_inst((pcpos + 1) as u32);
    let mut scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::DO_LEN, reg_ra, reg_rb);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
