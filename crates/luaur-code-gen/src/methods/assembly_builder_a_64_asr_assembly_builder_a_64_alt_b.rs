use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn asr_register_a_64_register_a_64_u8(
        &mut self,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: u8,
    ) {
        let size = if dst.kind() == crate::enums::kind_a_64::KindA64::x {
            64
        } else {
            32
        };

        debug_assert!((src2 as i32) < size);

        self.place_bfm(
            b"asr".as_ptr() as *const core::ffi::c_char,
            dst,
            src1,
            src2 as i32,
            0b00_100110,
            src2 as i32,
            size - 1,
        );
    }
}
