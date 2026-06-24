use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ubfx(&mut self, dst: RegisterA64, src: RegisterA64, f: u8, w: u8) {
        let size = if dst.kind() == crate::enums::kind_a_64::KindA64::x {
            64
        } else {
            32
        };

        CODEGEN_ASSERT!(w > 0 && f as i32 + w as i32 <= size);

        // f * 100 + w is only used for disassembly printout; in the future we might replace it with two separate fields for readability
        self.place_bfm(
            c"ubfx".as_ptr(),
            dst,
            src,
            (f as i32) * 100 + (w as i32),
            0b10_100110,
            f as i32,
            f as i32 + w as i32 - 1,
        );
    }
}
