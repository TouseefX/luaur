use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn log_c_char_register_a_64_address_a_64(
        &mut self,
        opcode: *const core::ffi::c_char,
        dst: RegisterA64,
        src: AddressA64,
    ) {
        self.log_append(format_args!(" {:<12}", unsafe {
            core::ffi::CStr::from_ptr(opcode).to_string_lossy()
        }));
        self.log_register_a_64(dst);
        self.text.push(',');
        self.log_address_a_64(src);
        self.text.push('\n');
    }
}
