use crate::enums::ir_cmd::IrCmd;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_capture_type::LuauCaptureType;
use luaur_common::enums::luau_opcode::LuauOpcode;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;
use luaur_common::macros::luau_insn_d::LUAU_INSN_D;
use luaur_common::macros::luau_insn_op::LUAU_INSN_OP;
use luaur_common::macros::luau_unreachable::LUAU_UNREACHABLE;
use luaur_vm::enums::lua_type::lua_Type;

#[allow(non_snake_case)]
pub fn translate_inst_new_closure(build: &mut IrBuilder, pc: *const Instruction, pcpos: i32) {
    let pc_val = unsafe { *pc };
    let d = LUAU_INSN_D(pc_val) as u32;
    CODEGEN_ASSERT!(d < unsafe { (*build.function.proto).sizep as u32 });

    let ra = LUAU_INSN_A(pc_val) as u8;
    let pv = unsafe { *(*build.function.proto).p.add(d as usize) };

    let savedpc_op = build.const_uint((pcpos + 1) as u32);
    build.inst_ir_cmd_ir_op(IrCmd::SET_SAVEDPC, savedpc_op);

    let env = build.inst_ir_cmd(IrCmd::LOAD_ENV);
    let nups_op = build.const_uint(unsafe { (*pv).nups as u32 });
    let d_op = build.const_uint(d);
    let ncl = build.inst_ir_cmd_ir_op_ir_op_ir_op(IrCmd::NEWCLOSURE, nups_op, env, d_op);

    let ra_op = build.vm_reg(ra);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, ra_op, ncl);
    let function_tag = build.const_tag(lua_Type::LUA_TFUNCTION as u8);
    build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, ra_op, function_tag);

    let nups = unsafe { (*pv).nups as i32 };
    for ui in 0..nups {
        let uinsn = unsafe { *pc.add(ui as usize + 1) };
        CODEGEN_ASSERT!(LUAU_INSN_OP(uinsn) == LuauOpcode::LOP_CAPTURE as u32);

        let capture_type = LUAU_INSN_A(uinsn) as u8;
        match capture_type {
            x if x == LuauCaptureType::LCT_VAL as u8 => {
                let reg_src = build.vm_reg(LUAU_INSN_B(uinsn) as u8);
                let src = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, reg_src);
                let upvalue = build.vm_upvalue(ui as u8);
                let dst =
                    build.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_CLOSURE_UPVAL_ADDR, ncl, upvalue);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, dst, src);
            }
            x if x == LuauCaptureType::LCT_REF as u8 => {
                let reg_src = build.vm_reg(LUAU_INSN_B(uinsn) as u8);
                let src = build.inst_ir_cmd_ir_op(IrCmd::FINDUPVAL, reg_src);
                let upvalue = build.vm_upvalue(ui as u8);
                let dst =
                    build.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_CLOSURE_UPVAL_ADDR, ncl, upvalue);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_POINTER, dst, src);
                let upval_tag = build.const_tag(lua_Type::LUA_TUPVAL as u8);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TAG, dst, upval_tag);
            }
            x if x == LuauCaptureType::LCT_UPVAL as u8 => {
                let undef = build.undef();
                let src_upvalue = build.vm_upvalue(LUAU_INSN_B(uinsn) as u8);
                let src = build.inst_ir_cmd_ir_op_ir_op(
                    IrCmd::GET_CLOSURE_UPVAL_ADDR,
                    undef,
                    src_upvalue,
                );
                let dst_upvalue = build.vm_upvalue(ui as u8);
                let dst =
                    build.inst_ir_cmd_ir_op_ir_op(IrCmd::GET_CLOSURE_UPVAL_ADDR, ncl, dst_upvalue);
                let load = build.inst_ir_cmd_ir_op(IrCmd::LOAD_TVALUE, src);
                build.inst_ir_cmd_ir_op_ir_op(IrCmd::STORE_TVALUE, dst, load);
            }
            _ => {
                CODEGEN_ASSERT!(false, "Unknown upvalue capture type");
                LUAU_UNREACHABLE!();
            }
        }
    }

    build.inst_ir_cmd(IrCmd::CHECK_GC);
}
