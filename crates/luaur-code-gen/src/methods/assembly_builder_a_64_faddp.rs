use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn faddp(&mut self, dst: RegisterA64, src: RegisterA64) {
        CODEGEN_ASSERT!(dst.kind() == KindA64::d || dst.kind() == KindA64::s);
        CODEGEN_ASSERT!(dst.kind() == src.kind());

        let is_d = if dst.kind() == KindA64::d { 1 } else { 0 };
        let op = 0b011_11110_0_0_11000_01101_10 | (is_d << 12);

        self.place_r_1(
            b"faddp\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            op,
        );
    }
}
