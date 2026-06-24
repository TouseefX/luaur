use crate::enums::category_x_64::CategoryX64;
use crate::enums::size_x_64::SizeX64;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;

impl AssemblyBuilderX64 {
    pub fn vcvtsi2ss(&mut self, dst: OperandX64, src1: OperandX64, src2: OperandX64) {
        let is_qword = (if src2.cat == CategoryX64::reg {
            src2.base.size()
        } else {
            src2.memSize
        }) == SizeX64::qword;

        self.place_avx_c_char_operand_x_64_operand_x_64_operand_x_64_u8_bool_u8_u8(
            c"vcvtsi2ss".as_ptr(),
            dst,
            src1,
            src2,
            0x2a,
            is_qword,
            0x0F,
            0xF3,
        );
    }
}
