use crate::enums::kind::Kind;
use crate::functions::writef_32::writef_32;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::patch::Patch;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn adr_register_a_64_f32(&mut self, dst: RegisterA64, value: f32) {
        let pos = self.allocate_data(4, 4);
        let location = self.get_code_size();

        unsafe {
            let p = self.data.as_mut_ptr().add(pos);
            writef_32(p, value);
        }

        self.place_adr_c_char_register_a_64_u8(core::ptr::null(), dst, 0b10000);

        let data_size = self.data.len();
        let data_words = (data_size - pos) / 4;
        let patch_value = location.wrapping_neg() as i32 - data_words as i32;

        self.patch_offset(location, patch_value, Kind::Imm19);
    }
}
