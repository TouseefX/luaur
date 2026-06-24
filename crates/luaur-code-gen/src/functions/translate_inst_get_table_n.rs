use crate::enums::ir_cmd::IrCmd;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_c::LUAU_INSN_C;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_get_table_n(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let insn = unsafe { *pc };
    let ra = LUAU_INSN_A(insn) as u8;
    let rb = LUAU_INSN_B(insn) as u8;
    let c = LUAU_INSN_C(insn) as u8;

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    if is_userdata_bytecode_type(bc_types.a) {
        let savedpc_arg = build.const_uint((pcpos + 1) as u32);
        build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        let c_plus_one = build.const_uint((c + 1) as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_TABLE, reg_ra, reg_rb, c_plus_one);
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

    let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, reg_rb);

    let c_int = build.const_int(c as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_ARRAY_SIZE, vb, c_int, fallback);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_NO_METATABLE, vb, fallback);

    let zero = build.const_int(0);
    let arr_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_ARR_ADDR, vb, zero);

    let c_times_sizeof = build.const_int(
        (c as i32) * (core::mem::size_of::<luaur_vm::type_aliases::t_value::TValue>() as i32),
    );
    let arr_el_tval = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, arr_el, c_times_sizeof);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, arr_el_tval);

    let next = build.block_at_inst((pcpos + 1) as u32);

    let mut scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    let savedpc_arg = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let c_plus_one = build.const_uint((c + 1) as u32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_TABLE, reg_ra, reg_rb, c_plus_one);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
