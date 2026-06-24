use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn rev(&mut self, dst: RegisterA64, src: RegisterA64) {
        CODEGEN_ASSERT!(dst.kind() == KindA64::w || dst.kind() == KindA64::x);
        CODEGEN_ASSERT!(dst.kind() == src.kind());

        self.place_r_1(
            b"rev\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            0b10_11010110_00000_0000_10 | (dst.kind() == KindA64::x) as u32,
        );
    }
}
