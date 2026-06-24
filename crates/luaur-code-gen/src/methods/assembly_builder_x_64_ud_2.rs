use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
    pub fn ud_2(&mut self) {
        if self.log_text {
            self.log_c_char(c"ud2".as_ptr() as *const core::ffi::c_char);
        }

        self.place(0x0f);
        self.place(0x0b);
    }
}
