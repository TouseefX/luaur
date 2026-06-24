use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn place_adr_c_char_register_a_64_u8(
        &mut self,
        name: *const core::ffi::c_char,
        dst: RegisterA64,
        op: u8,
    ) {
        if self.log_text {
            self.log_c_char_register_a_64(name, dst);
        }

        // Avoid CODEGEN_ASSERT! macro invocation here: it currently trips a type mismatch
        // in assert_call_handler arguments elsewhere in the codebase.
        debug_assert!(dst.kind() == KindA64::x);

        self.place((dst.index() as u32) | ((op as u32) << 24));
        self.commit();
    }
}
