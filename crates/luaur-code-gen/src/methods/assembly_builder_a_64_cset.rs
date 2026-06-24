use crate::enums::condition_a_64::ConditionA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn cset(&mut self, dst: RegisterA64, cond: ConditionA64) {
        CODEGEN_ASSERT!(
            dst.kind() == crate::enums::kind_a_64::KindA64::x
                || dst.kind() == crate::enums::kind_a_64::KindA64::w
        );

        let src = if dst.kind() == crate::enums::kind_a_64::KindA64::x {
            RegisterA64::xzr
        } else {
            RegisterA64::wzr
        };

        self.place_cs(
            b"cset\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src,
            src,
            cond,
            0b11010_10_0,
            0b01,
            1,
        );
    }
}
