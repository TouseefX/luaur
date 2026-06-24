use crate::enums::ir_cmd::IrCmd;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::records::bytecode_types::BytecodeTypes;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_c::LUAU_INSN_C;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_get_table(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let insn = unsafe { *pc };
    let ra = LUAU_INSN_A(insn) as u8;
    let rb = LUAU_INSN_B(insn) as u8;
    let rc = LUAU_INSN_C(insn) as u8;

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    if is_userdata_bytecode_type(bc_types.a)
        || bc_types.b == LuauBytecodeType::LBC_TYPE_STRING.0 as u8
    {
        let savedpc_arg = build.const_uint((pcpos + 1) as u32);
        build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        let reg_rc = build.vm_reg(rc);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_TABLE, reg_ra, reg_rb, reg_rc);
        return;
    }

    let fallback = build.fallback_block(pcpos as u32);

    let reg_rb = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
    let const_tag_table = build.const_tag(lua_Type::LUA_TTABLE as u8);
    let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
        build.vm_exit(pcpos as u32)
    } else {
        fallback
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, const_tag_table, exit_or_fallback);

    let reg_rc = build.vm_reg(rc);
    let tc = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rc);
    let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    let exit_or_fallback_c = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
        build.vm_exit(pcpos as u32)
    } else {
        fallback
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tc, const_tag_number, exit_or_fallback_c);

    let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, reg_rb);
    let vc = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_rc);

    let index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::TRY_NUM_TO_INDEX, vc, fallback);

    let const_int_one = build.const_int(1);
    let index = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_INT, index, const_int_one);

    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_ARRAY_SIZE, vb, index, fallback);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_NO_METATABLE, vb, fallback);

    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_ARR_ADDR, vb, index);

    let arr_el_tval = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, arr_el);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, arr_el_tval);

    let next = build.block_at_inst((pcpos + 1) as u32);

    let mut scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let reg_rc = build.vm_reg(rc);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_TABLE, reg_ra, reg_rb, reg_rc);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
