use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_aux_kv::LUAU_INSN_AUX_KV;
use luaur_common::macros::luau_insn_aux_not::LUAU_INSN_AUX_NOT;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::t_value::TValue;

pub fn translate_inst_jumpx_eq_n(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let aux = unsafe { *pc.add(1) };
    let not_ = LUAU_INSN_AUX_NOT(aux) != 0;

    let target = build.block_at_inst((pcpos + 1 + LUAU_INSN_D(unsafe { *pc })) as u32);
    let next = build.block_at_inst((pcpos + 2) as u32);
    let check_value = build.block(IrBlockKind::Internal);

    let vm_reg_ra = build.vm_reg(ra);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);

    let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_EQ_TAG,
        ta,
        const_tag_number,
        check_value,
        if not_ { target } else { next },
    );

    build.begin_block(check_value);
    let vm_reg_ra = build.vm_reg(ra);
    let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, vm_reg_ra);

    CODEGEN_ASSERT!(build.function.proto.is_null() == false);
    let protok: TValue = unsafe {
        *(*build.function.proto)
            .k
            .add(LUAU_INSN_AUX_KV(aux) as usize)
    };

    CODEGEN_ASSERT!(protok.tt == lua_Type::LUA_TNUMBER as i32);
    let vb = build.const_double(unsafe { protok.value.n });

    let cond = build.cond(IrCondition::NotEqual);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_CMP_NUM,
        va,
        vb,
        cond,
        if not_ { target } else { next },
        if not_ { next } else { target },
    );

    if build.is_internal_block(next) {
        build.begin_block(next);
    }
}
