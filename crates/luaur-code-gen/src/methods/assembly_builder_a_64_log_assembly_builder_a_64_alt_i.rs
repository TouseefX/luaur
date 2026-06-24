use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn log_c_char_register_a_64_label_i32(
        &mut self,
        opcode: *const core::ffi::c_char,
        src: RegisterA64,
        label: Label,
        imm: i32,
    ) {
        self.log_append(format_args!(" {:<12}", unsafe {
            core::ffi::CStr::from_ptr(opcode).to_string_lossy()
        }));
        self.log_register_a_64(src);
        self.text.push(',');
        if imm >= 0 {
            self.log_append(format_args!("#{},", imm));
        }
        self.log_append(format_args!(".L{}\n", label.id));
    }
}
