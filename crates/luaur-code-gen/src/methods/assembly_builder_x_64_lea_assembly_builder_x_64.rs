use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn lea_operand_x_64_operand_x_64(&mut self, mut lhs: OperandX64, mut rhs: OperandX64) {
        if self.log_text {
            self.log_c_char_operand_x_64_operand_x_64(c"lea".as_ptr(), lhs, rhs);
        }

        CODEGEN_ASSERT!(
            lhs.cat == CategoryX64::reg
                && rhs.cat == CategoryX64::mem
                && rhs.memSize == SizeX64::none
        );
        CODEGEN_ASSERT!(
            rhs.base == crate::records::register_x_64::RegisterX64::rip
                || rhs.base.size() == lhs.base.size()
        );
        CODEGEN_ASSERT!(
            rhs.index == crate::records::register_x_64::RegisterX64::noreg
                || rhs.index.size() == lhs.base.size()
        );

        rhs.memSize = lhs.base.size();

        self.place_binary_reg_and_reg_mem(lhs, rhs, 0x8d, 0x8d);
    }
}
