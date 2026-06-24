use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
    pub fn int3(&mut self) {
        if self.log_text {
            self.log_c_char(c"int3".as_ptr() as *const core::ffi::c_char);
        }

        self.place(0xcc);
        self.commit();
    }
}
