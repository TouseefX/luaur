use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fdiv(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
        if dst.kind() == KindA64::d {
            // Avoid CODEGEN_ASSERT! macro invocation: it expands through luaur_common::assert_call_handler
            // and currently creates type-mismatch issues in this translation set.
            debug_assert!(src1.kind() == KindA64::d && src2.kind() == KindA64::d);

            self.place_r_3(c"fdiv".as_ptr(), dst, src1, src2, 0b11110_01_1, 0b0001_10);
        } else if dst.kind() == KindA64::s {
            debug_assert!(src1.kind() == KindA64::s && src2.kind() == KindA64::s);

            self.place_r_3(c"fdiv".as_ptr(), dst, src1, src2, 0b11110_00_1, 0b0001_10);
        } else {
            debug_assert!(
                dst.kind() == KindA64::q && src1.kind() == KindA64::q && src2.kind() == KindA64::q
            );

            self.place_vr(c"fdiv".as_ptr(), dst, src1, src2, 0b1_01110_00_1, 0b11111_1);
        }
    }
}
