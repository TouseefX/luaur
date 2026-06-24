use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn tbnz(&mut self, src: RegisterA64, bit: u8, label: &mut Label) {
        self.place_btr(c"tbnz".as_ptr(), label, 0b0110111, src, bit);
    }
}
