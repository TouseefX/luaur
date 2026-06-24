use crate::enums::ir_block_kind::IrBlockKind;
use crate::enums::ir_cmd::IrCmd;
use crate::functions::get_op_length::get_op_length;
use crate::functions::is_userdata_bytecode_type::is_userdata_bytecode_type;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use core::ffi::{c_char, c_uint};
use luaur_common::enums::luau_bytecode_type::LuauBytecodeType;
use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_aux_kv_16::LUAU_INSN_AUX_KV16;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_c::LUAU_INSN_C;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::getstr::getstr;
use luaur_vm::macros::tsvalue::tsvalue;
use luaur_vm::records::g_cheader::GCheader;
use luaur_vm::records::t_string::TString;
use luaur_vm::type_aliases::tms::TMS;

#[repr(C)]
struct TStringHeader {
    hdr: GCheader,
    _padding1: [c_char; 1],
    atom: i16,
    _padding2: [c_char; 2],
    next: *mut TString,
    hash: c_uint,
    len: c_uint,
    data: [c_char; 1],
}

pub fn translate_inst_namecall(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) -> bool {
    let ra = LUAU_INSN_A(unsafe { *pc }) as u8;
    let rb = LUAU_INSN_B(unsafe { *pc }) as u8;

    let op = LUAU_INSN_OP(unsafe { *pc });
    let aux = if LuauOpcode::from(op as u8) == LuauOpcode::LOP_NAMECALLUDATA {
        LUAU_INSN_AUX_KV16(unsafe { *pc.add(1) })
    } else {
        unsafe { *pc.add(1) }
    } as u32;

    let bc_types = build.function.get_bytecode_types_at(pcpos);

    if bc_types.a == LuauBytecodeType::LBC_TYPE_VECTOR.0 as u8 {
        let reg_rb = build.vm_reg(rb);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(reg_rb, lua_Type::LUA_TVECTOR as u8, exit);

        let vector_namecall = unsafe { (*build.host_hooks).vector_namecall };
        if let Some(vector_namecall) = vector_namecall {
            let call = unsafe { *pc.add(2) };
            let call_op = LuauOpcode::from(LUAU_INSN_OP(call) as u8);
            CODEGEN_ASSERT!(call_op == LuauOpcode::LOP_CALL || call_op == LuauOpcode::LOP_CALLFB);

            let callra = LUAU_INSN_A(call) as i32;
            let nparams = LUAU_INSN_B(call) as i32 - 1;
            let nresults = LUAU_INSN_C(call) as i32 - 1;

            let proto_k = unsafe { (*build.function.proto).k.add(aux as usize) };
            let ts = unsafe { tsvalue!(proto_k) };
            let field = unsafe { getstr(ts) };
            let len = unsafe { (*(ts as *const TStringHeader)).len } as usize;

            let handled = unsafe {
                vector_namecall(
                    build as *mut IrBuilder,
                    field,
                    len,
                    callra,
                    rb as i32,
                    nparams,
                    nresults,
                    pcpos,
                )
            };
            if handled {
                return true;
            }
        }

        let pcpos_op = build.const_uint(pcpos as u32);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        let aux_op = build.vm_const(aux);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::FALLBACK_NAMECALL,
            pcpos_op,
            reg_ra,
            reg_rb,
            aux_op,
        );
        return false;
    }

    if is_userdata_bytecode_type(bc_types.a) {
        let reg_rb = build.vm_reg(rb);
        let exit = build.vm_exit(pcpos as u32);
        build.load_and_check_tag(reg_rb, lua_Type::LUA_TUSERDATA as u8, exit);

        let pcpos_op = build.const_uint(pcpos as u32);
        let reg_ra = build.vm_reg(ra);
        let reg_rb = build.vm_reg(rb);
        let aux_op = build.vm_const(aux);
        build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
            IrCmd::FALLBACK_NAMECALL,
            pcpos_op,
            reg_ra,
            reg_rb,
            aux_op,
        );
        return false;
    }

    let next = build.block_at_inst((pcpos + get_op_length(LuauOpcode::LOP_NAMECALL) as i32) as u32);
    let fallback = build.fallback_block(pcpos as u32);
    let first_fast_path_success = build.block(IrBlockKind::Internal);
    let second_fast_path = build.block(IrBlockKind::Internal);

    let exit_or_fallback = if bc_types.a == LuauBytecodeType::LBC_TYPE_TABLE.0 as u8 {
        build.vm_exit(pcpos as u32)
    } else {
        fallback
    };
    let reg_rb = build.vm_reg(rb);
    build.load_and_check_tag(reg_rb, lua_Type::LUA_TTABLE as u8, exit_or_fallback);
    let reg_rb = build.vm_reg(rb);
    let table = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, reg_rb);

    CODEGEN_ASSERT!(!build.function.proto.is_null());
    let proto_k = unsafe { (*build.function.proto).k.add(aux as usize) };
    let ts = unsafe { tsvalue!(proto_k) };
    let hash = unsafe { (*(ts as *const TStringHeader)).hash };
    let hash_op = build.const_uint(hash as u32);
    let addr_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_HASH_NODE_ADDR, table, hash_op);

    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::JUMP_SLOT_MATCH,
        addr_node_el,
        aux_op,
        first_fast_path_success,
        second_fast_path,
    );

    build.begin_block(first_fast_path_success);
    let reg_self = build.vm_reg(ra.wrapping_add(1));
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, reg_self, table);
    let reg_self = build.vm_reg(ra.wrapping_add(1));
    let table_tag = build.const_tag(lua_Type::LUA_TTABLE as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_self, table_tag);

    let offset_val = build.const_int(0);
    let node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, addr_node_el, offset_val);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, node_el);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    build.begin_block(second_fast_path);

    build.inst_ir_cmd_ir_op_ir_op(IrCmd::CHECK_NODE_NO_NEXT, addr_node_el, fallback);

    let tm_index = build.const_int(TMS::TM_INDEX as i32);
    let index_ptr =
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::TRY_CALL_FASTGETTM, table, tm_index, fallback);

    build.load_and_check_tag(index_ptr, lua_Type::LUA_TTABLE as u8, fallback);
    let index = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, index_ptr);

    let pcpos_op = build.const_uint(pcpos as u32);
    let aux_op = build.vm_const(aux);
    let addr_index_node_el =
        build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::GET_SLOT_NODE_ADDR, index, pcpos_op, aux_op);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op(
        IrCmd::CHECK_SLOT_MATCH,
        addr_index_node_el,
        aux_op,
        fallback,
    );

    let reg_rb = build.vm_reg(rb);
    let table2 = build.inst_ir_cmd_ir_op(IrCmd::LOAD_POINTER, reg_rb);
    let reg_self = build.vm_reg(ra.wrapping_add(1));
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, reg_self, table2);
    let reg_self = build.vm_reg(ra.wrapping_add(1));
    let table_tag = build.const_tag(lua_Type::LUA_TTABLE as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, reg_self, table_tag);

    let zero = build.const_int(0);
    let index_node_el = build.inst_ir_cmd_ir_op_ir_op(IrCmd::LOAD_TVALUE, addr_index_node_el, zero);
    let reg_ra = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, reg_ra, index_node_el);
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    build.begin_block(fallback);
    let pcpos_op = build.const_uint(pcpos as u32);
    let reg_ra = build.vm_reg(ra);
    let reg_rb = build.vm_reg(rb);
    let aux_op = build.vm_const(aux);
    build.inst_ir_cmd_ir_op_ir_op_ir_op_ir_op(
        IrCmd::FALLBACK_NAMECALL,
        pcpos_op,
        reg_ra,
        reg_rb,
        aux_op,
    );
    build.inst_ir_cmd_ir_op(IrCmd::JUMP, next);

    build.begin_block(next);

    false
}
