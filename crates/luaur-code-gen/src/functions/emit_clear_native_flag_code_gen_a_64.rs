use crate::enums::address_kind_a_64::AddressKindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::macros::lua_callinfo_native::LUA_CALLINFO_NATIVE;
use luaur_vm::records::call_info::CallInfo;
use luaur_vm::records::lua_state::lua_State;

const X0: RegisterA64 = RegisterA64 { bits: (0 << 3) | 2 };
const X1: RegisterA64 = RegisterA64 { bits: (1 << 3) | 2 };
const X2: RegisterA64 = RegisterA64 { bits: (2 << 3) | 2 };
const X19: RegisterA64 = RegisterA64 {
    bits: (19 << 3) | 2,
};

pub fn emit_clear_native_flag_assembly_builder_a_64(build: &mut AssemblyBuilderA64) {
    let r_state = X19;
    let r_ci = X0;
    let r_flags = X1;
    let r_mask = X2;

    build.ldr(
        r_ci,
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            r_state,
            core::mem::offset_of!(lua_State, ci) as i32,
            AddressKindA64::imm,
        ),
    );
    build.ldr(
        r_flags,
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            r_ci,
            core::mem::offset_of!(CallInfo, flags) as i32,
            AddressKindA64::imm,
        ),
    );
    build.mov_register_a_64_i32(r_mask, !LUA_CALLINFO_NATIVE as i32);
    build.and_register_a_64_register_a_64_register_a_64_i32(r_flags, r_flags, r_mask, 0);
    build.str(
        r_flags,
        AddressA64::address_a_64_register_a_64_i32_address_kind_a_64(
            r_ci,
            core::mem::offset_of!(CallInfo, flags) as i32,
            AddressKindA64::imm,
        ),
    );
}
