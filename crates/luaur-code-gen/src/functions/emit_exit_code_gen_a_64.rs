use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::native_context::NativeContext;
use crate::records::register_a_64::RegisterA64;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const X0: RegisterA64 = reg(KindA64::x, 0);
const X1: RegisterA64 = reg(KindA64::x, 1);
const R_NATIVE_CONTEXT: RegisterA64 = reg(KindA64::x, 20);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

pub fn emit_exit_assembly_builder_a_64_bool(build: &mut AssemblyBuilderA64, continue_in_vm: bool) {
    build.mov_register_a_64_i32(X0, continue_in_vm as i32);
    build.ldr(
        X1,
        mem(
            R_NATIVE_CONTEXT,
            core::mem::offset_of!(NativeContext, gateExit) as i32,
        ),
    );
    build.br(X1);
}
