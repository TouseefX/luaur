use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn tbz(&mut self, src: RegisterA64, bit: u8, label: &mut Label) {
        self.place_btr(c"tbz".as_ptr(), label, 0b0110110, src, bit);
    }
}
