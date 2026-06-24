use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fmul(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
        if dst.kind() == KindA64::d {
            // CODEGEN_ASSERT! currently expands through luaur_common::assert_call_handler with a
            // pointer-vs-&str mismatch in this translation set, so use debug_assert! instead.
            debug_assert!(src1.kind() == KindA64::d && src2.kind() == KindA64::d);

            self.place_r_3(c"fmul".as_ptr(), dst, src1, src2, 0b11110_01_1, 0b0000_10);
        } else if dst.kind() == KindA64::s {
            debug_assert!(src1.kind() == KindA64::s && src2.kind() == KindA64::s);

            self.place_r_3(c"fmul".as_ptr(), dst, src1, src2, 0b11110_00_1, 0b0000_10);
        } else {
            debug_assert!(
                dst.kind() == KindA64::q && src1.kind() == KindA64::q && src2.kind() == KindA64::q
            );

            self.place_vr(c"fmul".as_ptr(), dst, src1, src2, 0b1_01110_00_1, 0b11011_1);
        }
    }
}
