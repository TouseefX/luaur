use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::macros::call_fallback_yield::CALL_FALLBACK_YIELD;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::module_helpers::ModuleHelpers;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::records::closure::{Closure, LClosure};
use luaur_vm::records::proto::Proto;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const X0: RegisterA64 = reg(KindA64::x, 0);
const X1: RegisterA64 = reg(KindA64::x, 1);
const X2: RegisterA64 = reg(KindA64::x, 2);
const R_CONSTANTS: RegisterA64 = reg(KindA64::x, 23);
const R_CODE: RegisterA64 = reg(KindA64::x, 24);
const R_CLOSURE: RegisterA64 = reg(KindA64::x, 25);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

pub fn emitContinueCall(build: &mut AssemblyBuilderA64, helpers: &mut ModuleHelpers) {
    crate::CODEGEN_ASSERT!(CALL_FALLBACK_YIELD == 1);

    build.tbnz(X0, 0, &mut helpers.exitNoContinueVm);

    build.ldr(
        X1,
        mem(
            X0,
            (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p)) as i32,
        ),
    );

    build.ldr(X2, mem(X1, core::mem::offset_of!(Proto, exectarget) as i32));
    build.cbz(X2, &mut helpers.exitContinueVm);

    build.mov_register_a_64_register_a_64(R_CLOSURE, X0);
    build.ldp(
        R_CONSTANTS,
        R_CODE,
        mem(X1, core::mem::offset_of!(Proto, k) as i32),
    );
    build.br(X2);
}
