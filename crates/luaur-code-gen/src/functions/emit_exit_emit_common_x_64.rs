use crate::enums::size_x_64::SizeX64;
use crate::functions::dword_reg::dword_reg;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::native_context::NativeContext;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

// C++ EmitCommonX64.h: `constexpr RegisterX64 rNativeContext = r13;`
const fn r_native_context() -> RegisterX64 {
    RegisterX64 {
        bits: (13u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

pub fn emit_exit(build: &mut AssemblyBuilderX64, continue_in_vm: bool) {
    let eax = dword_reg(RegisterX64::rax);

    if continue_in_vm {
        build.mov(OperandX64::reg(eax), OperandX64::imm(1));
    } else {
        build.xor_(OperandX64::reg(eax), OperandX64::reg(eax));
    }

    build.jmp_operand_x_64(OperandX64::mem(
        SizeX64::qword,
        RegisterX64::noreg,
        1,
        r_native_context(),
        core::mem::offset_of!(NativeContext, gateExit) as i32,
    ));
}
