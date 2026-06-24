use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::type_aliases::t_value::TValue;

const K_MAX_IMMEDIATE: usize = (1 << 12) - 1;
const K_TEMP_SLOTS: u32 = 1;
const S_TEMPORARY_DATA: i32 = 9 * 8;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const X0: RegisterA64 = reg(KindA64::x, 0);
const X1: RegisterA64 = reg(KindA64::x, 1);
const X20: RegisterA64 = reg(KindA64::x, 20);
const X25: RegisterA64 = reg(KindA64::x, 25);
const SP: RegisterA64 = reg(KindA64::none, 31);
const D0: RegisterA64 = reg(KindA64::d, 0);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

pub fn emit_invoke_libm_1_p(build: &mut AssemblyBuilderA64, func: usize, arg: i32) {
    CODEGEN_ASSERT!(K_TEMP_SLOTS >= 1);
    CODEGEN_ASSERT!(S_TEMPORARY_DATA as usize <= K_MAX_IMMEDIATE);

    let tvalue_size = core::mem::size_of::<TValue>() as i32;
    let value_offset = core::mem::offset_of!(TValue, value) as i32;

    build.ldr(D0, mem(X25, arg * tvalue_size + value_offset));
    build.add_register_a_64_register_a_64_u16(X0, SP, S_TEMPORARY_DATA as u16);
    build.ldr(X1, mem(X20, func as i32));
    build.blr(X1);
}
