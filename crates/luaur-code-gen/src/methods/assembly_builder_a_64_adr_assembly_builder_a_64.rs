use crate::enums::kind::Kind;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::patch::Patch;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn adr_register_a_64_void_usize(
        &mut self,
        dst: RegisterA64,
        ptr: *const core::ffi::c_void,
        size: usize,
    ) {
        let pos = self.allocate_data(size, 4);
        let location = self.get_code_size();

        unsafe {
            core::ptr::copy_nonoverlapping(ptr as *const u8, self.data.as_mut_ptr().add(pos), size);
        }

        self.place_adr_c_char_register_a_64_u8(c"adr".as_ptr(), dst, 0b10000);

        let data_size = self.data.len();
        let offset = -(location as i32) - (((data_size - pos) / 4) as i32);
        self.patch_offset(location, offset, Kind::Imm19);
    }
}
