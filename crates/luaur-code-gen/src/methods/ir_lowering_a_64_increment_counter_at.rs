use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::functions::emit_add_offset::emit_add_offset;
use crate::records::address_a_64::AddressA64;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::records::closure::{Closure, LClosure};
use luaur_vm::records::proto::Proto;

const fn reg(kind: KindA64, index: u8) -> RegisterA64 {
    RegisterA64 {
        bits: kind as u8 | (index << 3),
    }
}

const R_CLOSURE: RegisterA64 = reg(KindA64::x, 23);

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_increment_counter_at(&mut self, offset: usize) {
        let temp1 = self.regs.alloc_temp(KindA64::x);
        let temp2 = self.regs.alloc_temp(KindA64::x);

        unsafe {
            (*self.build).ldr(
                temp1,
                mem(
                    R_CLOSURE,
                    (core::mem::offset_of!(Closure, inner) + core::mem::offset_of!(LClosure, p))
                        as i32,
                ),
            );
            (*self.build).ldr(
                temp1,
                mem(temp1, core::mem::offset_of!(Proto, execdata) as i32),
            );

            let counter_offset = ((*(*self.function).proto).sizecode as usize + offset) * 4;
            emit_add_offset(&mut *self.build, temp2, temp1, counter_offset);

            (*self.build).ldr(temp1, mem(temp2, 0));
            (*self.build).add_register_a_64_register_a_64_u16(temp1, temp1, 1);
            (*self.build).str(temp1, mem(temp2, 0));
        }

        self.regs.free_temp(temp1);
        self.regs.free_temp(temp2);
    }
}
