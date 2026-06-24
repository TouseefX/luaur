use crate::enums::ir_cmd::IrCmd;
use crate::functions::get_initialized_fallback::get_initialized_fallback;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::tms::TMS;

pub fn translate_inst_minus(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let bc_types = build.function.get_bytecode_types_at(pcpos);

    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

    if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
        let reg_rb = build.vm_reg(rb);
        let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
        let const_tag_vector = build.const_tag(lua_Type::LUA_TVECTOR as u8);
        let vm_exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, const_tag_vector, vm_exit);

        let reg_rb = build.vm_reg(rb);
        let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, reg_rb);
        let va = build.inst_ir_cmd_ir_op(IrCmd::UNM_VEC, vb);
        let va = build.inst_ir_cmd_ir_op(IrCmd::TAG_VECTOR, va);
        let reg_ra = build.vm_reg(ra);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, va);
        return;
    }

    if is_userdata_bytecode_type(bc_types.a) {
        let savedpc_arg = build.const_uint((pcpos + 1) as u32);
        build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        let const_int_unm = build.const_int(TMS::TM_UNM as i32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::DO_ARITH,
            reg_ra,
            reg_rb,
            reg_rb,
            const_int_unm,
        );
        return;
    }

    let mut fallback = crate::records::ir_op::IrOp::ir_op();

    let reg_rb = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
    let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
        build.vm_exit(pcpos as u32)
    } else {
        get_initialized_fallback(build, &mut fallback, pcpos)
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, const_tag_number, exit_or_fallback);

    let reg_rb = build.vm_reg(rb);
    let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_rb);
    let va = build.inst_ir_cmd_ir_op(IrCmd::UNM_NUM, vb);

    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_DOUBLE, reg_ra, va);

    if ra != rb {
        let reg_ra = build.vm_reg(ra);
        let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_ra, const_tag_number);
    }

    if fallback.kind() != crate::enums::ir_op_kind::IrOpKind::None {
        let next = build.block_at_inst((pcpos + 1) as u32);
        let mut scope = FallbackStreamScope::new(build, fallback, next);
        let build = &mut *scope.build;

        let savedpc_arg = build.const_uint((pcpos + 1) as u32);
        build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_arg);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        let const_int_unm = build.const_int(TMS::TM_UNM as i32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::DO_ARITH,
            reg_ra,
            reg_rb,
            reg_rb,
            const_int_unm,
        );
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
    }
}
