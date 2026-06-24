use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn place_i12(
        &mut self,
        name: *const core::ffi::c_char,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: i32,
        op: u8,
    ) {
        if self.log_text {
            self.log_c_char_register_a_64_register_a_64_i32(name, dst, src1, src2);
        }

        // Avoid CODEGEN_ASSERT! here: it currently expands to luaur_common::assert_call_handler
        // which expects *const i8 parameters, but the macro supplies &str.
        // Keep the same logical checks using plain Rust asserts instead.

        assert!(dst.kind() == KindA64::w || dst.kind() == KindA64::x || dst == RegisterA64::sp);
        assert!(
            dst.kind() == src1.kind()
                || (dst.kind() == KindA64::x && src1 == RegisterA64::sp)
                || (dst == RegisterA64::sp && src1.kind() == KindA64::x)
        );
        assert!(src2 >= 0 && src2 < (1 << 12));

        let sf = if dst.kind() != KindA64::w {
            0x80000000
        } else {
            0
        };

        self.place(
            (dst.index() as u32)
                | ((src1.index() as u32) << 5)
                | ((src2 as u32) << 10)
                | ((op as u32) << 24)
                | sf,
        );
        self.commit();
    }
}
