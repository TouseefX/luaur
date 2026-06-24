use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;

use crate::macros::op_plus_cc::OP_PLUS_CC;

impl AssemblyBuilderX64 {
    pub fn place_jcc(&mut self, name: *const core::ffi::c_char, label: &mut Label, cc: u8) {
        self.place(0x0f);
        self.place(OP_PLUS_CC(0x80, cc));
        self.place_label(label);

        if self.log_text {
            self.log_c_char_label(name, *label);
        }

        self.commit();
    }
}
