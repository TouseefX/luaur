use crate::enums::size_x_64::SizeX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::macros::lua_callinfo_native::LUA_CALLINFO_NATIVE;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::lua_state::lua_State;

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

pub fn emit_clear_native_flag(build: &mut AssemblyBuilderX64) {
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
    build.and_(
        OperandX64::mem(
            SizeX64::dword,
            RegisterX64::noreg,
            1,
            RegisterX64::rax,
            core::mem::offset_of!(CallInfo, flags) as i32,
        ),
        OperandX64::imm(!LUA_CALLINFO_NATIVE),
    );
}
