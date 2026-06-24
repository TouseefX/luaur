use crate::enums::size_x_64::SizeX64;
use crate::functions::luau_reg_tag::luau_reg_tag;
use crate::functions::luau_reg_value::luau_reg_value;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::ir_op::IrOp;
use crate::records::ir_reg_alloc_x_64::IrRegAllocX64;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::enums::lua_type::lua_Type;

const fn reg(index: u8, size: SizeX64) -> RegisterX64 {
    RegisterX64 {
        bits: (index << RegisterX64::INDEX_SHIFT) | size as u8,
    }
}

const R_NATIVE_CONTEXT: RegisterX64 = reg(13, SizeX64::qword);
const XMM0: RegisterX64 = reg(0, SizeX64::xmmword);

fn s_temporary_slot() -> OperandX64 {
    OperandX64::mem(SizeX64::qword, RegisterX64::noreg, 1, RegisterX64::rsp, 0)
}

pub fn emit_builtin_math_frexp(
    regs: &mut IrRegAllocX64,
    build: &mut AssemblyBuilderX64,
    ra: i32,
    arg: i32,
    nresults: i32,
) {
    let mut call_wrap = IrCallWrapperX64::ir_call_wrapper_x_64_ir_call_wrapper_x_64(
        regs,
        build,
        k_invalid_inst_idx,
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::xmmword,
        luau_reg_value(arg),
        IrOp::ir_op(),
    );
    call_wrap.add_argument_size_x_64_operand_x_64_ir_op(
        SizeX64::qword,
        s_temporary_slot(),
        IrOp::ir_op(),
    );
    call_wrap.call(&OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        R_NATIVE_CONTEXT,
        core::mem::offset_of!(NativeContext, libm_frexp) as i32,
    ));

    build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra), OperandX64::reg(XMM0));
    build.mov(
        luau_reg_tag(ra),
        OperandX64::imm(lua_Type::LUA_TNUMBER as i32),
    );

    if nresults > 1 {
        build.vcvtsi2sd(
            OperandX64::reg(XMM0),
            OperandX64::reg(XMM0),
            OperandX64::mem(SizeX64::dword, RegisterX64::noreg, 1, RegisterX64::rsp, 0),
        );
        build.vmovsd_operand_x_64_operand_x_64(luau_reg_value(ra + 1), OperandX64::reg(XMM0));
        build.mov(
            luau_reg_tag(ra + 1),
            OperandX64::imm(lua_Type::LUA_TNUMBER as i32),
        );
    }
}
