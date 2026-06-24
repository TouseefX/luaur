use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
    pub fn log_c_char(&mut self, opcode: *const core::ffi::c_char) {
        self.log_append(format_args!(" {}\n", unsafe {
            core::ffi::CStr::from_ptr(opcode).to_string_lossy()
        }));
    }
}
