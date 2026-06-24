use crate::enums::size_x_64::SizeX64;
use crate::functions::emit_update_base_emit_common_x_64::emit_update_base;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

// C++ EmitCommonX64.h: `constexpr RegisterX64 rConstants = r12;`
const fn r_constants() -> RegisterX64 {
    RegisterX64 {
        bits: (12u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

// C++ EmitCommonX64.h: `constexpr RegisterX64 rNativeContext = r13;`
const fn r_native_context() -> RegisterX64 {
    RegisterX64 {
        bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

const K_STACK_OFFSET_TO_LOCALS: i32 = 16 + 32;

fn s_code() -> OperandX64 {
    OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        RegisterX64::rsp,
        K_STACK_OFFSET_TO_LOCALS + 8,
    )
}

const SIZEOF_INSTRUCTION: i32 = 4;

pub fn emit_fallback(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    offset: i32,
    pcpos: i32,
) {
    // fallback(L, instruction, base, k)
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

    let reg = call_wrap.suggest_next_argument_register(SizeX64::qword);
    build.mov(OperandX64::reg(reg), s_code());
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        OperandX64::mem(
            SizeX64::none,
            RegisterX64::noreg,
            1,
            reg,
            pcpos * SIZEOF_INSTRUCTION,
        ),
        IrOp::ir_op(),
    );

    // rBase = rbp in this crate.
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        OperandX64::reg(RegisterX64::rbp),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        OperandX64::reg(r_constants()),
        IrOp::ir_op(),
    );

    call_wrap.call(&OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        r_native_context(),
        offset,
    ));

    emit_update_base(build);
}
