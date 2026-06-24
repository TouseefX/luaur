use crate::enums::ir_cmd::IrCmd;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::records::fallback_stream_scope::FallbackStreamScope;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::type_aliases::lua_node::LuaNode;

pub fn translate_inst_set_table_ks(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let insn = unsafe { *pc };
    let ra = LUAU_INSN_A(insn) as u8;
    let rb = LUAU_INSN_B(insn) as u8;

    let aux = if LuauOpcode::from(LUAU_INSN_OP(insn) as u8) == LuauOpcode::LOP_SETUDATAKS {
        LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
    } else {
        unsafe { *pc.add(1) }
    };

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    let rb_reg = build.vm_reg(rb);
    let tb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TAG, rb_reg);

    if is_userdata_bytecode_type(bc_types.a) {
        let userdata_tag = build.const_tag(lua_Type::LUA_TUSERDATA as u8);
        let exit = build.vm_exit(pcpos as u32);
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, userdata_tag, exit);

        let pcpos_op = build.const_uint(pcpos as u32);
        let ra_op = build.vm_reg(ra);
        let rb_op = build.vm_reg(rb);
        let aux_op = build.vm_const(aux);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::FALLBACK_SETTABLEKS,
            pcpos_op,
            ra_op,
            rb_op,
            aux_op,
        );
        return;
    }

    let fallback = build.fallback_block(pcpos as u32);
    let table_tag = build.const_tag(lua_Type::LUA_TTABLE as u8);
    let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
        build.vm_exit(pcpos as u32)
    } else {
        fallback
    };
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_TAG, tb, table_tag, exit_or_fallback);

    let vb = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, rb_reg);

    let pcpos_op = build.const_uint(pcpos as u32);
    let aux_op = build.vm_const(aux);
    let addr_slot_el =
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, vb, pcpos_op, aux_op);

    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::CHECK_SLOT_MATCH, addr_slot_el, aux_op, fallback);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_READONLY, vb, fallback);

    let ra_op = build.vm_reg(ra);
    let tva = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, ra_op);
    let offset = build.const_int(core::mem::offset_of!(LuaNode, val) as i32);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::STORE_TVALUE, addr_slot_el, tva, offset);

    let undef = build.undef();
    build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::BARRIER_TABLE_FORWARD, vb, ra_op, undef);

    let next = build.block_at_inst((pcpos + 2) as u32);
    let mut scope = FallbackStreamScope::new(build, fallback, next);
    let build = &mut *scope.build;

    let pcpos_op = build.const_uint(pcpos as u32);
    let ra_op = build.vm_reg(ra);
    let rb_op = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::FALLBACK_SETTABLEKS,
        pcpos_op,
        ra_op,
        rb_op,
        aux_op,
    );
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);
}
