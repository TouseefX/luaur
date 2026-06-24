use crate::enums::size_x_64::SizeX64;
use crate::functions::writeu_32::writeu_32;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::operand_x_64::OperandX64;
use crate::records::register_x_64::RegisterX64;

impl AssemblyBuilderX64 {
    pub fn u32x4(&mut self, x: u32, y: u32, z: u32, w: u32) -> OperandX64 {
        let pos = {
            if self.data_pos < 16 {
                let old_size = self.data.len();
                self.data.resize(self.data.len() * 2, 0);

                unsafe {
                    core::ptr::copy_nonoverlapping(
                        self.data.as_ptr(),
                        self.data.as_mut_ptr().add(old_size),
                        old_size,
                    );
                    core::ptr::write_bytes(self.data.as_mut_ptr(), 0, old_size);
                }

                self.data_pos += old_size;
            }

            self.data_pos = (self.data_pos - 16) & !(16 - 1);
            self.data_pos
        };

        unsafe {
            writeu_32(self.data.as_mut_ptr().add(pos), x);
            writeu_32(self.data.as_mut_ptr().add(pos + 4), y);
            writeu_32(self.data.as_mut_ptr().add(pos + 8), z);
            writeu_32(self.data.as_mut_ptr().add(pos + 12), w);
        }

        OperandX64::operand_x_64_size_x_64_register_x_64_u8_register_x_64_i32(
            SizeX64::xmmword,
            RegisterX64::noreg,
            1,
            RegisterX64::rip,
            (pos as i32) - (self.data.len() as i32),
        )
    }
}
