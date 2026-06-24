use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::kind_a_64::KindA64;
use crate::functions::cast_reg::cast_reg;
use crate::records::address_a_64::AddressA64;
use crate::records::ir_block::IrBlock;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::type_aliases::closure::Closure as ClosureAlias;
use luaur_vm::type_aliases::lua_table::LuaTable as LuaTableAlias;

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
    pub fn ir_lowering_a_64_check_safe_env(&mut self, target: IrOp, index: u32, next: &IrBlock) {
        let _ = next;
        let mut fresh = Label { id: 0, location: 0 };
        let temp: RegisterA64 = self.regs.alloc_temp(KindA64::x);
        let tempw: RegisterA64 = cast_reg(KindA64::w, temp);
        unsafe {
            let offset_env = core::mem::offset_of!(ClosureAlias, env);
            let offset_safeenv = core::mem::offset_of!(LuaTableAlias, safeenv);
            (*self.build).ldr(temp, mem(R_CLOSURE, offset_env as i32));
            (*self.build).ldrb(tempw, mem(temp, offset_safeenv as i32));
            (*self.build).cbz(
                tempw,
                self.ir_lowering_a_64_get_target_label(target, index, &mut fresh),
            );
            self.ir_lowering_a_64_finalize_target_label(target, index, &mut fresh);
        }
    }
}
