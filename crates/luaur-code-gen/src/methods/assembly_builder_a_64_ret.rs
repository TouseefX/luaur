use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
    pub fn ret(&mut self) {
        self.place_0(c"ret".as_ptr(), 0b1101011_0_0_10_11111_0000_0_0_11110_00000);
    }
}
