use crate::enums::kind::Kind;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;

impl AssemblyBuilderA64 {
    pub fn patch_offset(&mut self, location: u32, value: i32, kind: Kind) {
        let offset = if kind == Kind::Imm26 { 0 } else { 5 };
        let range = match kind {
            Kind::Imm19 => 1 << 19,
            Kind::Imm26 => 1 << 26,
            Kind::Imm14 => 1 << 14,
        };

        debug_assert!((self.code[location as usize] & (((range - 1) as u32) << offset)) == 0);

        if value > -(range >> 1) && value < (range >> 1) {
            self.code[location as usize] |= ((value as u32) & (range - 1) as u32) << offset;
        } else {
            self.overflowed = true;
        }
    }
}
