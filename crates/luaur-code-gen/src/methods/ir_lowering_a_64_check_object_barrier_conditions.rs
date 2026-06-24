use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::condition_a_64::ConditionA64;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::kind_a_64::KindA64;
use crate::functions::cast_reg::cast_reg;
use crate::functions::is_gco::is_gco;
use crate::records::address_a_64::AddressA64;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::macros::bitmask::bit2mask;
use luaur_vm::macros::blackbit::BLACKBIT;
use luaur_vm::macros::white_0_bit::WHITE0BIT;
use luaur_vm::macros::white_1_bit::WHITE1BIT;
use luaur_vm::records::g_cheader::GCheader;
use luaur_vm::type_aliases::t_value::TValue;

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_check_object_barrier_conditions(
        &mut self,
        object: RegisterA64,
        temp: RegisterA64,
        ra: RegisterA64,
        ra_op: IrOp,
        ratag: i32,
        skip: &mut Label,
    ) {
        let tempw = cast_reg(KindA64::w, temp);

        unsafe {
            if ratag == -1 || !is_gco(ratag as u8) {
                if ra_op.kind() == IrOpKind::Inst {
                    (*self.build).umov_4s(tempw, ra, 3);
                } else {
                    let addr = self.ir_lowering_a_64_temp_addr(
                        ra_op,
                        core::mem::offset_of!(TValue, tt) as i32,
                        temp,
                    );
                    (*self.build).ldr(tempw, addr);
                }

                (*self.build).cmp_register_a_64_u16(tempw, lua_Type::LUA_TSTRING as u16);
                (*self.build).b_condition_a_64_label(ConditionA64::Less, skip);
            }

            (*self.build).ldrb(
                tempw,
                mem(object, core::mem::offset_of!(GCheader, marked) as i32),
            );
            (*self.build).tbz(tempw, BLACKBIT, skip);

            if ra_op.kind() == IrOpKind::Inst {
                (*self.build).fmov_register_a_64_register_a_64(temp, cast_reg(KindA64::d, ra));
            } else {
                let addr = self.ir_lowering_a_64_temp_addr(
                    ra_op,
                    core::mem::offset_of!(TValue, value) as i32,
                    temp,
                );
                (*self.build).ldr(temp, addr);
            }

            (*self.build).ldrb(
                tempw,
                mem(temp, core::mem::offset_of!(GCheader, marked) as i32),
            );
            (*self.build)
                .tst_register_a_64_u32(tempw, bit2mask(WHITE0BIT, WHITE1BIT as i32) as u32);
            (*self.build).b_condition_a_64_label(ConditionA64::Equal, skip);
        }
    }
}
