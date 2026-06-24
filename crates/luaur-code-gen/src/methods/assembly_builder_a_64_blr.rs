use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn blr(&mut self, src: RegisterA64) {
        self.place_br(c"blr".as_ptr(), src, 0b1101011_0_0_01_11111_0000_0_0);
    }
}
