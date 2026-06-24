use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn cbz(&mut self, src: RegisterA64, label: &mut Label) {
        self.place_bcr(c"cbz".as_ptr(), label, 0b011010_0, src);
    }
}
