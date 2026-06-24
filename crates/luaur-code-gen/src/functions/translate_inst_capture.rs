use crate::enums::ir_cmd::IrCmd;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_builder::IrBuilder;
use crate::type_aliases::instruction_ir_translation::Instruction;
use luaur_common::enums::luau_capture_type::LuauCaptureType;
use luaur_common::macros::luau_insn_a::LUAU_INSN_A;
use luaur_common::macros::luau_insn_b::LUAU_INSN_B;

pub fn translate_inst_capture(build: &mut IrBuilder, pc: *const Instruction, _pcpos: i32) {
    let type_ = LUAU_INSN_A(unsafe { *pc }) as u8;
    let index = LUAU_INSN_B(unsafe { *pc }) as u8;

    match type_ {
        x if x == LuauCaptureType::LCT_VAL as u8 => {
            let reg = build.vm_reg(index as u8);
            let const_uint = build.const_uint(0);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, reg, const_uint);
        }
        x if x == LuauCaptureType::LCT_REF as u8 => {
            let reg = build.vm_reg(index as u8);
            let const_uint = build.const_uint(1);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, reg, const_uint);
        }
        x if x == LuauCaptureType::LCT_UPVAL as u8 => {
            let upvalue = build.vm_upvalue(index as u8);
            let const_uint = build.const_uint(0);
            build.inst_ir_cmd_ir_op_ir_op(IrCmd::CAPTURE, upvalue, const_uint);
        }
        _ => {
            CODEGEN_ASSERT!(false, "Unknown upvalue capture type");
        }
    }
}
