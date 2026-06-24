use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl OperandX64 {
    pub fn operand_x_64_i32(imm: i32) -> Self {
        OperandX64 {
            cat: CategoryX64::imm,
            index: RegisterX64 { bits: 0xFF },
            base: RegisterX64 { bits: 0xFF },
            memSize: SizeX64::none,
            scale: 1,
            imm,
        }
    }
}
