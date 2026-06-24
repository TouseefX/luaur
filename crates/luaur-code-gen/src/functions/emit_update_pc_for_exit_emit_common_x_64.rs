use crate::enums::size_x_64::SizeX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::lua_state::lua_State;

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

// C++ EmitCommonX64.h: `sCode = qword[rsp + kStackOffsetToLocals + 8]`
// kStackOffsetToLocals = kStackExtraArgumentStorage(16) + kStackRegHomeStorage(32) = 48
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

pub fn emit_update_pc_for_exit(build: &mut AssemblyBuilderX64) {
    // edx = pcpos * sizeof(Instruction)
    build.add(OperandX64::reg(RegisterX64::rdx), s_code());
    build.mov(
        OperandX64::reg(RegisterX64::rax),
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            r_state(),
            core::mem::offset_of!(lua_State, ci) as i32,
        ),
    );
    build.mov(
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            RegisterX64::rax,
            core::mem::offset_of!(CallInfo, savedpc) as i32,
        ),
        OperandX64::reg(RegisterX64::rdx),
    );
}
