use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn adr_register_a_64_label(&mut self, dst: RegisterA64, label: &mut Label) {
        self.place_adr_c_char_register_a_64_u8_label(
            b"adr\0".as_ptr() as *const core::ffi::c_char,
            dst,
            0b10000,
            label,
        );
    }
}
