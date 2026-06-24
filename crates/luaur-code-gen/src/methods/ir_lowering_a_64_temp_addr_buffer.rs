use crate::enums::address_kind_a_64::AddressKindA64;
use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::kind_a_64::KindA64;
use crate::functions::emit_add_offset::emit_add_offset;
use crate::functions::produces_dirty_high_register_bits::produces_dirty_high_register_bits;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::address_a_64::AddressA64;
use crate::records::ir_lowering_a_64::IrLoweringA64;
use crate::records::ir_op::IrOp;
use crate::records::register_a_64::RegisterA64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::records::luau_buffer::LuauBuffer;
use luaur_vm::records::udata::Udata;

fn mem(base: RegisterA64, data: i32) -> AddressA64 {
    AddressA64 {
        kind: AddressKindA64::imm,
        base,
        offset: RegisterA64::noreg,
        data,
    }
}

impl IrLoweringA64 {
    pub fn ir_lowering_a_64_temp_addr_buffer(
        &mut self,
        buffer_op: IrOp,
        index_op: IrOp,
        tag: u8,
    ) -> AddressA64 {
        CODEGEN_ASSERT!(tag == lua_Type::LUA_TUSERDATA as u8 || tag == lua_Type::LUA_TBUFFER as u8);

        let data_offset = if tag == lua_Type::LUA_TBUFFER as u8 {
            (core::mem::size_of::<LuauBuffer>() - core::mem::size_of::<[core::ffi::c_char; 1]>())
                as i32
        } else {
            core::mem::offset_of!(Udata, data) as i32
        };

        if index_op.kind() == IrOpKind::Inst {
            unsafe {
                CODEGEN_ASSERT!(!produces_dirty_high_register_bits(
                    (*self.function).inst_op(index_op).cmd
                ));
            }

            let temp = self.regs.alloc_temp(KindA64::x);
            let buffer = self.ir_lowering_a_64_reg_op(buffer_op);
            let index = self.ir_lowering_a_64_reg_op(index_op);
            unsafe {
                (*self.build)
                    .add_register_a_64_register_a_64_register_a_64_i32(temp, buffer, index, 0);
            }
            return mem(temp, data_offset);
        } else if index_op.kind() == IrOpKind::Constant {
            let buffer = self.ir_lowering_a_64_reg_op(buffer_op);
            let index = unsafe { (*self.function).int_op(index_op) };

            if (index as u32).wrapping_add(data_offset as u32) <= 255 {
                return mem(buffer, index + data_offset);
            }

            if index < 0 {
                return mem(buffer, data_offset);
            }

            let temp = self.regs.alloc_temp(KindA64::x);
            unsafe {
                emit_add_offset(&mut *self.build, temp, buffer, index as usize);
            }
            return mem(temp, data_offset);
        }

        CODEGEN_ASSERT!(false, "Unsupported instruction form");
        mem(RegisterA64::noreg, 0)
    }
}
