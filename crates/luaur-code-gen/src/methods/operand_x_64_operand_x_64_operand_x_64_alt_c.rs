use crate::enums::size_x_64::SizeX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl OperandX64 {
    pub fn operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
        size: SizeX64,
        index: RegisterX64,
        scale: u8,
        base: RegisterX64,
        disp: i32,
    ) -> Self {
        OperandX64 {
            cat: crate::enums::category_x_64::CategoryX64::mem,
            index,
            base,
            memSize: size,
            scale,
            imm: disp,
        }
    }
}
