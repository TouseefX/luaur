use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::functions::is_expected_or_unknown_bytecode_type::is_expected_or_unknown_bytecode_type;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_builder::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;

pub fn translate_inst_jump_if_eq(
    build: &mut IrBuilder,
    pc: *const Instruction,
    pcpos: i32,
    not_: bool,
) {
    let pc_val = unsafe { *pc };
    let ra = LUAU_INSN_A(pc_val) as u8;
    let rb = unsafe { *pc.add(1) } as u8;

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(pc_val)) as u32);
    let next = build.block_at_inst((pcpos + 2) as u32);

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    if is_expected_or_unknown_bytecode_type(bc_types.a, LuauBytecodeType::LBC_TYPE_NUMBER)
        && is_expected_or_unknown_bytecode_type(bc_types.b, LuauBytecodeType::LBC_TYPE_NUMBER)
    {
        let fallback = build.fallback_block(pcpos as u32);

        let reg_ra = build.vm_reg(ra);
        let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_ra);
        let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, ta, number_tag, fallback);

        let reg_rb = build.vm_reg(rb);
        let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, reg_rb);
        let number_tag = build.const_tag(lua_Type::LUA_TNUMBER as u8);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, number_tag, fallback);

        let reg_ra = build.vm_reg(ra);
        let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_ra);
        let reg_rb = build.vm_reg(rb);
        let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, reg_rb);

        let cond = build.cond(IrCondition::NotEqual);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
            IrCmd::JUMP_CMP_NUM,
            va,
            vb,
            cond,
            if not_ { target } else { next },
            if not_ { next } else { target },
        );

        build.begin_block(fallback);
    }

    let savedpc = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc);

    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let cond = build.cond(IrCondition::Equal);
    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CMP_ANY, reg_ra, reg_rb, cond);
    let zero = build.const_int(0);
    let cond = build.cond(IrCondition::Equal);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_CMP_INT,
        result,
        zero,
        cond,
        if not_ { target } else { next },
        if not_ { next } else { target },
    );

    build.begin_block(next);
}
