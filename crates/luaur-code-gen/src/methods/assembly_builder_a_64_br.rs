use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn br(&mut self, src: RegisterA64) {
        self.place_br(c"br".as_ptr(), src, 0b1101011_0_0_00_11111_0000_0_0);
    }
}
