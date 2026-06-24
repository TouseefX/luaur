use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;

impl AssemblyBuilderX64 {
    pub fn log_c_char_label(&mut self, opcode: *const core::ffi::c_char, label: Label) {
        self.log_append(format_args!(
            " {:<12}.L{}\n",
            unsafe { core::ffi::CStr::from_ptr(opcode).to_string_lossy() },
            label.id
        ));
    }
}
