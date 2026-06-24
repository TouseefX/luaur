use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl OperandX64 {
    pub fn operand_x_64_operator_index(&self, mut addr: OperandX64) -> OperandX64 {
        CODEGEN_ASSERT!(self.cat == CategoryX64::mem);
        CODEGEN_ASSERT!(
            self.index == RegisterX64::noreg
                && self.scale == 1
                && self.base == RegisterX64::noreg
                && self.imm == 0
        );
        CODEGEN_ASSERT!(addr.memSize == SizeX64::none);

        addr.cat = CategoryX64::mem;
        addr.memSize = self.memSize;
        addr
    }
}
