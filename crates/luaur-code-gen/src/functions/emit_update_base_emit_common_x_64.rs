use crate::enums::size_x_64::SizeX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::records::lua_state::lua_State;

// C++ EmitCommonX64.h: `constexpr RegisterX64 rState = r15;`
const fn r_state() -> RegisterX64 {
    RegisterX64 {
        bits: (15u8 << RegisterX64::INDEX_SHIFT) | SizeX64::qword as u8,
    }
}

pub fn emit_update_base(build: &mut AssemblyBuilderX64) {
    // rBase = L->base. In this crate the stack base register is `rbp` (see luau_reg helpers).
    build.mov(
        OperandX64::reg(RegisterX64::rbp),
        OperandX64::mem(
            SizeX64::qword,
            RegisterX64::noreg,
            1,
            r_state(),
            core::mem::offset_of!(lua_State, base) as i32,
        ),
    );
}
