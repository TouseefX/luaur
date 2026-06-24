use crate::enums::size_x_64::SizeX64;
use crate::records::ir_call_wrapper_x_64::IrCallWrapperX64;
use crate::records::ir_data::k_invalid_inst_idx;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl IrCallWrapperX64 {
    pub fn rename_conflicting_register(&mut self, conflict: RegisterX64) {
        // Get a fresh register
        let fresh_reg = unsafe { (*self.regs).alloc_reg(conflict.size(), k_invalid_inst_idx) };

        if conflict.size() == SizeX64::xmmword {
            unsafe {
                (*self.build).vmovsd_operand_x_64_operand_x_64_operand_x_64(
                    OperandX64::reg(fresh_reg),
                    OperandX64::reg(conflict),
                    OperandX64::reg(conflict),
                );
            }
        } else {
            unsafe {
                (*self.build).mov(OperandX64::reg(fresh_reg), OperandX64::reg(conflict));
            }
        }

        self.rename_source_registers(conflict, fresh_reg);
    }
}
