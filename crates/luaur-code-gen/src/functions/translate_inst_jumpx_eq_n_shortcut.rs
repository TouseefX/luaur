use crate::enums::ir_cmd::IrCmd;
use crate::enums::ir_condition::IrCondition;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::records::ir_op::IrOp;
use crate::type_aliases::instruction_ir_builder::Instruction;

use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_aux_kv::LUAU_INSN_AUX_KV;
use luaur_common::macros::luau_insn_aux_not::LUAU_INSN_AUX_NOT;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::t_value::TValue;

pub fn translate_inst_jumpx_eq_n_shortcut(
    build: &mut IrBuilder,
    pc: *const Instruction,
    pcpos: i32,
) {
    let rr = LUAU_INSN_A(unsafe { *(pc.add(2) as *const u32) });

    let ra = LUAU_INSN_A(unsafe { *(pc as *const u32) });
    let aux = unsafe { *(pc.add(1) as *const u32) };
    let not_ = LUAU_INSN_AUX_NOT(aux) != 0;

    let next = build.block_at_inst((pcpos + 4) as u32);

    let vm_reg_ra = build.vm_reg(ra as u8);
    let ta = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, vm_reg_ra);
    let va = build.inst_ir_cmd_ir_op(IrCmd::LOAD_DOUBLE, vm_reg_ra);

    CODEGEN_ASSERT!(build.function.proto.is_null() == false);
    let aux_kv = LUAU_INSN_AUX_KV(aux) as usize;
    let protok = unsafe { *(*build.function.proto).k.add(aux_kv) };
    let protok_value_n = unsafe { protok.value.n };

    CODEGEN_ASSERT!(protok.tt == lua_Type::LUA_TNUMBER as i32);
    let vb = build.const_double(protok_value_n);

    let const_tag_number = build.const_tag(lua_Type::LUA_TNUMBER as u8);
    let cond = if not_ {
        IrCondition::NotEqual
    } else {
        IrCondition::Equal
    };
    let cond_op = build.cond(cond);

    let result = build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op_ir_op(
        IrCmd::CMP_SPLIT_TVALUE,
        ta,
        const_tag_number,
        va,
        vb,
        cond_op,
    );

    let vm_reg_rr = build.vm_reg(rr as u8);
    let const_tag_boolean = build.const_tag(lua_Type::LUA_TBOOLEAN as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, vm_reg_rr, const_tag_boolean);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_INT, vm_reg_rr, result);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    if build.is_internal_block(next) {
        build.begin_block(next);
    }
}
