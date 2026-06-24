use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn movk(&mut self, dst: RegisterA64, src: u16, shift: i32) {
        self.place_i16(c"movk".as_ptr(), dst, src as i32, 0b11100101, shift);
    }
}
