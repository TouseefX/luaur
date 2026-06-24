use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::enums::ir_op_kind::IrOpKind;
use crate::functions::get_initialized_fallback::get_initialized_fallback;
use crate::functions::is_expected_or_unknown_bytecode_type::is_expected_or_unknown_bytecode_type;
use crate::records::bytecode_types::BytecodeTypes;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::FFlag;
use luaur_vm::enums::lua_type::lua_Type;

use luaur_common::macros::luau_insn_a::LUAU_INSN_A;

pub fn translate_inst_jump_if_eq_shortcut(
    build: &mut IrBuilder,
    pc: *const Instruction,
    pcpos: i32,
    not_: bool,
) {
    let rr = LUAU_INSN_A(unsafe { *(pc.add(2) as *const u32) });

    let ra = LUAU_INSN_A(unsafe { *pc });
    let rb = unsafe { *(pc.add(1) as *const u32) };

    let next = build.block_at_inst((pcpos + 4) as u32);
    let mut fallback = IrOp::ir_op();

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    // fast-path: number (when both operands are expected to be a number or are unknown)
    if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
        && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
    {
        let vm_reg_ra = build.vm_reg(ra as u8);
        let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);
        let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        let fallback_op = if bc_types.a == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
            build.vm_exit(pcpos as u32)
        } else {
            get_initialized_fallback(build, &mut fallback, pcpos)
        };
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, ta, const_tag_number, fallback_op);

        let vm_reg_rb = build.vm_reg(rb as u8);
        let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_rb);
        let const_tag_number_2 = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        let fallback_op_2 = if bc_types.b == LuauBytecodeType::LBC_TYPE_NUMBER.0 as u8 {
            build.vm_exit(pcpos as u32)
        } else {
            get_initialized_fallback(build, &mut fallback, pcpos)
        };
        build.inst_ir_cmd_ir_op_ir_op_ir_op(
            IrCmd::CHECK_TAG,
            tb,
            const_tag_number_2,
            fallback_op_2,
        );

        let reg_ra = build.vm_reg(ra as u8);
        let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_ra);
        let reg_rb = build.vm_reg(rb as u8);
        let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_rb);

        let const_tag_number_3 = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        let const_tag_number_4 = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        let cond = if not_ {
            IrCondition::NotEqual
        } else {
            IrCondition::Equal
        };
        let cond_op = build.cond(cond);

        let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CMP_SPLIT_TVALUE,
            const_tag_number_3,
            const_tag_number_4,
            va,
            vb,
            cond_op,
        );

        let reg_rr = build.vm_reg(rr as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, reg_rr, result);
        let reg_rr = build.vm_reg(rr as u8);
        let boolean_tag = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_rr, boolean_tag);
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

        // If we don't need a fallback, we are done
        if fallback.kind() == IrOpKind::None {
            return;
        }

        // Otherwise, start the fallback block
        // Note that if the number fast-path is not taken at all code that would have been in the fallback is actually the main path
        build.begin_block(fallback);
    } else if FFlag::LuauCodegenInteger2.get()
        && is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_INTEGER)
        && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_INTEGER)
    {
        let vm_reg_ra = build.vm_reg(ra as u8);
        let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);
        let const_tag_integer = build.const_tag(lua_Type::LUA_TINTEGER as u8);
        let fallback_op = if bc_types.a == LuauBytecodeType::LBC_TYPE_INTEGER.0 as u8 {
            build.vm_exit(pcpos as u32)
        } else {
            get_initialized_fallback(build, &mut fallback, pcpos)
        };
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, ta, const_tag_integer, fallback_op);

        let vm_reg_rb = build.vm_reg(rb as u8);
        let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_rb);
        let const_tag_integer_2 = build.const_tag(lua_Type::LUA_TINTEGER as u8);
        let fallback_op_2 = if bc_types.b == LuauBytecodeType::LBC_TYPE_INTEGER.0 as u8 {
            build.vm_exit(pcpos as u32)
        } else {
            get_initialized_fallback(build, &mut fallback, pcpos)
        };
        build.inst_ir_cmd_ir_op_ir_op_ir_op(
            IrCmd::CHECK_TAG,
            tb,
            const_tag_integer_2,
            fallback_op_2,
        );

        let reg_ra = build.vm_reg(ra as u8);
        let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, reg_ra);
        let reg_rb = build.vm_reg(rb as u8);
        let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_INT64, reg_rb);

        let const_tag_integer_3 = build.const_tag(lua_Type::LUA_TINTEGER as u8);
        let const_tag_integer_4 = build.const_tag(lua_Type::LUA_TINTEGER as u8);
        let cond = if not_ {
            IrCondition::NotEqual
        } else {
            IrCondition::Equal
        };
        let cond_op = build.cond(cond);

        let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::CMP_SPLIT_TVALUE,
            const_tag_integer_3,
            const_tag_integer_4,
            va,
            vb,
            cond_op,
        );

        let reg_rr = build.vm_reg(rr as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, reg_rr, result);
        let reg_rr = build.vm_reg(rr as u8);
        let boolean_tag = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_rr, boolean_tag);
        build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

        // If we don't need a fallback, we are done
        if fallback.kind() == IrOpKind::None {
            return;
        }

        // Otherwise, start the fallback block
        // Note that if the number fast-path is not taken at all code that would have been in the fallback is actually the main path
        build.begin_block(fallback);
    }

    let savedpc = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc);

    let reg_ra = build.vm_reg(ra as u8);
    let reg_rb = build.vm_reg(rb as u8);
    let cond = build.cond(IrCondition::Equal);
    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_ANY, reg_ra, reg_rb, cond);

    // CMP_ANY doesn't support NotEqual, but we can compute !result as 1-result
    if not_ {
        let const_int_1 = build.const_int(1);
        let result_2 = build.inst_ir_cmd_ir_op_ir_op(IrCmd::SUB_INT, const_int_1, result);
        let reg_rr = build.vm_reg(rr as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, reg_rr, result_2);
    } else {
        let reg_rr = build.vm_reg(rr as u8);
        build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, reg_rr, result);
    }

    let reg_rr = build.vm_reg(rr as u8);
    let boolean_tag = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_rr, boolean_tag);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
