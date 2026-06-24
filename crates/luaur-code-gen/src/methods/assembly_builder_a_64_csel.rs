use crate::enums::condition_a_64::ConditionA64;
use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn csel(
        &mut self,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: RegisterA64,
        cond: ConditionA64,
    ) {
        debug_assert!(dst.kind() == KindA64::x || dst.kind() == KindA64::w);

        self.place_cs(
            b"csel\0".as_ptr() as *const core::ffi::c_char,
            dst,
            src1,
            src2,
            cond,
            0b11010100,
            0b00,
            0,
        );
    }
}
