use crate::enums::condition_a_64::ConditionA64;
use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fcsel(
        &mut self,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: RegisterA64,
        cond: ConditionA64,
    ) {
        CODEGEN_ASSERT!(dst.kind() == src1.kind() && src1.kind() == src2.kind());
        CODEGEN_ASSERT!(dst.kind() == KindA64::d || dst.kind() == KindA64::s);

        if src1.kind() == KindA64::d {
            self.place_cs(
                b"fcsel\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src1,
                src2,
                cond,
                0b11110_01_1,
                0b11,
                0,
            );
        } else {
            self.place_cs(
                b"fcsel\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src1,
                src2,
                cond,
                0b11110_00_1,
                0b11,
                0,
            );
        }
    }
}
