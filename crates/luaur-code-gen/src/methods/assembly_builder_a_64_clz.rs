use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn clz(&mut self, dst: RegisterA64, src: RegisterA64) {
        debug_assert!(dst.kind() == KindA64::w || dst.kind() == KindA64::x);
        debug_assert!(dst.kind() == src.kind());

        self.place_r_1(
            b"clz\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            0b10_11010110_00000_00010_0,
        );
    }
}
