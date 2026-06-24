use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl OperandX64 {
    pub fn operand_x_64_register_x_64(reg: RegisterX64) -> Self {
        Self {
            cat: CategoryX64::reg,
            index: RegisterX64::noreg,
            base: reg,
            memSize: SizeX64::none,
            scale: 1,
            imm: 0,
        }
    }
}
