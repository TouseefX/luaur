use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fmov_register_a_64_f32(&mut self, dst: RegisterA64, src: f32) {
        debug_assert!(
            dst.kind() == crate::enums::kind_a_64::KindA64::s
                || dst.kind() == crate::enums::kind_a_64::KindA64::q
        );

        let imm = crate::functions::get_fmov_imm_fp_32::get_fmov_imm_fp_32(src);
        debug_assert!(imm >= 0 && imm <= 256);

        // fmov can't encode 0, but movi can; movi is otherwise not useful for fp immediates because it encodes repeating patterns
        if dst.kind() == crate::enums::kind_a_64::KindA64::s {
            if imm == 256 {
                self.place_fmov(
                    b"movi\0".as_ptr() as *const core::ffi::c_char,
                    dst,
                    src as f64,
                    0b001_0111100000_000_1110_01_00000,
                );
            } else {
                self.place_fmov(
                    b"fmov\0".as_ptr() as *const core::ffi::c_char,
                    dst,
                    src as f64,
                    0b000_11110_00_1_00000000_100_00000 | ((imm as u32) << 8),
                );
            }
        } else {
            if imm == 256 {
                self.place_fmov(
                    b"movi.4s\0".as_ptr() as *const core::ffi::c_char,
                    dst,
                    src as f64,
                    0b010_0111100000_000_0000_01_00000,
                );
            } else {
                self.place_fmov(
                    b"fmov.4s\0".as_ptr() as *const core::ffi::c_char,
                    dst,
                    src as f64,
                    0b010_0111100000_000_1111_0_1_00000
                        | (((imm as u32) >> 5) << 11)
                        | (imm as u32 & 31),
                );
            }
        }
    }
}
