use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;

impl AssemblyBuilderX64 {
    pub fn jmp_label(&mut self, label: &mut Label) {
        self.place(0xe9);
        self.place_label(label);

        if self.log_text {
            self.log_c_char_label(c"jmp".as_ptr(), *label);
        }

        self.commit();
    }
}
