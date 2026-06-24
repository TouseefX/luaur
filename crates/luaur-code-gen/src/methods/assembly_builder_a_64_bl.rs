use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;

impl AssemblyBuilderA64 {
    pub fn bl(&mut self, label: &mut Label) {
        self.place_b(
            b"bl\0".as_ptr() as *const core::ffi::c_char,
            label,
            0b1_00101,
        );
    }
}
