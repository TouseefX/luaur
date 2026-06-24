use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ucvtf(&mut self, dst: RegisterA64, src: RegisterA64) {
        // CODEGEN_ASSERT! currently expands to luaur_common::assert_call_handler(...),
        // which expects raw pointers. Avoid invoking it here.
        debug_assert!(dst.kind() == KindA64::d || dst.kind() == KindA64::s);
        debug_assert!(src.kind() == KindA64::w || src.kind() == KindA64::x);

        if dst.kind() == KindA64::d {
            self.place_r_1(
                b"ucvtf\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b000_11110_01_1_00_011_000000,
            );
        } else {
            self.place_r_1(
                b"ucvtf\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b000_11110_00_1_00_011_000000,
            );
        }
    }
}
