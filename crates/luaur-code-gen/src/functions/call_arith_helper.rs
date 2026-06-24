use crate::enums::size_x_64::SizeX64;
use crate::functions::emit_update_base_emit_common_x_64::emit_update_base;
use crate::functions::luau_reg_address::luau_reg_address;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::type_aliases::tms::TMS;

const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

const fn r_native_context() -> RegisterX64 {
    RegisterX64 {
        bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

pub fn call_arith_helper(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    ra: i32,
    b: OperandX64,
    c: OperandX64,
    tm: TMS,
) {
    let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
        regs,
        build,
        k_invalid_inst_idx,
    );

    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        OperandX64::reg(r_state()),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        luau_reg_address(ra),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(SizeX64::qword, b, IrOp::ir_op());
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(SizeX64::qword, c, IrOp::ir_op());

    let off = match tm {
        TMS::TM_ADD => core::mem::offset_of!(NativeContext, luaV_doarithadd),
        TMS::TM_SUB => core::mem::offset_of!(NativeContext, luaV_doarithsub),
        TMS::TM_MUL => core::mem::offset_of!(NativeContext, luaV_doarithmul),
        TMS::TM_DIV => core::mem::offset_of!(NativeContext, luaV_doarithdiv),
        TMS::TM_IDIV => core::mem::offset_of!(NativeContext, luaV_doarithidiv),
        TMS::TM_MOD => core::mem::offset_of!(NativeContext, luaV_doarithmod),
        TMS::TM_POW => core::mem::offset_of!(NativeContext, luaV_doarithpow),
        TMS::TM_UNM => core::mem::offset_of!(NativeContext, luaV_doarithunm),
        _ => {
            CODEGEN_ASSERT!(false); // "Invalid doarith helper operation tag"
            0
        }
    } as i32;

    call_wrap.call(&OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        r_native_context(),
        off,
    ));

    emit_update_base(build);
}
