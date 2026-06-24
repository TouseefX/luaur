use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn place_vr(
        &mut self,
        name: *const core::ffi::c_char,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: RegisterA64,
        op: u16,
        op2: u8,
    ) {
        if self.log_text {
            self.log_append(format_args!(
                " {:<12}v{}.4s,v{}.4s,v{}.4s\n",
                unsafe { core::ffi::CStr::from_ptr(name) }.to_string_lossy(),
                dst.index(),
                src1.index(),
                src2.index()
            ));
        }

        // Avoid CODEGEN_ASSERT! macro invocation: it expands through luaur_common::assert_call_handler
        // and currently creates type-mismatch issues in this translation set.
        debug_assert!(dst.kind() == KindA64::q);
        debug_assert!(dst.kind() == src1.kind() && dst.kind() == src2.kind());

        self.place(
            (dst.index() as u32)
                | ((src1.index() as u32) << 5)
                | ((op2 as u32) << 10)
                | ((src2.index() as u32) << 16)
                | ((op as u32) << 21)
                | (1u32 << 30),
        );
        self.commit();
    }
}
