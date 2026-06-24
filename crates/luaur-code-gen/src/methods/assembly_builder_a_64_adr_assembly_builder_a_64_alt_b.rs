use crate::enums::kind::Kind;
use crate::functions::writeu_64::writeu_64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn adr_register_a_64_u64(&mut self, dst: RegisterA64, value: u64) {
        let pos = self.allocate_data(8, 8);
        let location = self.get_code_size();

        unsafe {
            let p = self.data.as_mut_ptr().add(pos);
            writeu_64(p, value);
        }

        self.place_adr_c_char_register_a_64_u8(c"adr".as_ptr(), dst, 0b10000);

        let data_size = self.data.len();
        let data_words = (data_size - pos) / 4;
        let patch_value = location.wrapping_neg() as i32 - data_words as i32;

        self.patch_offset(location, patch_value, Kind::Imm19);
    }
}
