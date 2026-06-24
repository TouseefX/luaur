use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn udiv(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
        if self.log_text {
            self.log_c_char_register_a_64_register_a_64_register_a_64_i32(
                c"udiv".as_ptr(),
                dst,
                src1,
                src2,
                0,
            );
        }

        // Avoid CODEGEN_ASSERT! macro invocation here: it expands to luaur_common::assert_call_handler
        // which expects *const i8 parameters and may type-mismatch at call sites elsewhere.
        assert!(dst.kind() == KindA64::w || dst.kind() == KindA64::x);
        assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

        let sf: u32 = if dst.kind() == KindA64::x {
            0x80000000
        } else {
            0
        };

        self.place(
            (dst.index() as u32)
                | ((src1.index() as u32) << 5)
                | (0b000010 << 10)
                | ((src2.index() as u32) << 16)
                | (0b0011010110u32 << 21)
                | sf,
        );
        self.commit();
    }
}
