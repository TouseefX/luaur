use crate::enums::ir_op_kind::IrOpKind;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::ir_lowering_x_64::IrLoweringX64;
use crate::records::ir_op::IrOp;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::records::luau_buffer::LuauBuffer;
use luaur_vm::records::udata::Udata;

impl IrLoweringX64 {
    pub fn buffer_addr_op(&mut self, buffer_op: IrOp, index_op: IrOp, tag: u8) -> OperandX64 {
        CODEGEN_ASSERT!(tag == lua_Type::LUA_TUSERDATA as u8 || tag == lua_Type::LUA_TBUFFER as u8);
        let data_offset = if tag == lua_Type::LUA_TBUFFER as u8 {
            core::mem::size_of::<LuauBuffer>() - core::mem::size_of::<[core::ffi::c_char; 1]>()
        } else {
            core::mem::offset_of!(Udata, data)
        };

        if index_op.kind() == IrOpKind::Inst {
            let inst_op = unsafe { (*self.function).inst_op(index_op) };
            CODEGEN_ASSERT!(!crate::functions::produces_dirty_high_register_bits::produces_dirty_high_register_bits(inst_op.cmd));

            let buffer_reg = self.reg_op(buffer_op);
            let index_reg = self.reg_op(index_op);
            let scaled_index = crate::functions::qword_reg::qword_reg(index_reg);
            return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
                SizeX64::qword,
                scaled_index,
                1,
                buffer_reg,
                data_offset as i32,
            );
        } else if index_op.kind() == IrOpKind::Constant {
            let buffer_reg = self.reg_op(buffer_op);
            let index_val = self.int_op(index_op);
            return OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
                SizeX64::qword,
                RegisterX64::noreg,
                1,
                buffer_reg,
                index_val + data_offset as i32,
            );
        }

        CODEGEN_ASSERT!(false, "Unsupported instruction form");
        OperandX64::operand_x_64_register_x_64(RegisterX64::noreg)
    }
}
