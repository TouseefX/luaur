use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn bic(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64, shift: i32) {
        self.place_sr_3(
            b"bic\0" as *const _ as *const core::ffi::c_char,
            dst,
            src1,
            src2,
            0b00_01010,
            shift,
            1,
        );
    }
}
