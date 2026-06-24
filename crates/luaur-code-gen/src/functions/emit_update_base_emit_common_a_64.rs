use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::type_aliases::lua_state::lua_State;

use crate::records::register_a_64::RegisterA64;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const R_STATE: RegisterA64 = reg(KindA64::x, 19);
const R_BASE: RegisterA64 = reg(KindA64::x, 22);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

pub fn emit_update_base(build: &mut AssemblyBuilderA64) {
    build.ldr(
        R_BASE,
        mem(R_STATE, core::mem::offset_of!(lua_State, base) as i32),
    );
}
