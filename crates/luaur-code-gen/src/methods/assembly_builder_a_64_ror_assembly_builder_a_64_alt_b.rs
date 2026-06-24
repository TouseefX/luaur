use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ror_register_a_64_register_a_64_u8(
        &mut self,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: u8,
    ) {
        let size = if dst.kind() == KindA64::x { 64 } else { 32 };

        debug_assert!(src2 < size as u8);

        self.place_bfm(
            b"ror".as_ptr() as *const core::ffi::c_char,
            dst,
            src1,
            src2 as i32,
            0b00_100111,
            src1.index() as i32,
            src2 as i32,
        );
    }
}
