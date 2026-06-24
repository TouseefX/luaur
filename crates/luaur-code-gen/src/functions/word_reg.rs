use crate::enums::size_x_64::SizeX64;
use crate::records::register_x_64::RegisterX64;

pub const fn word_reg(reg: RegisterX64) -> RegisterX64 {
    RegisterX64 {
        bits: (SizeX64::word as u8) | (reg.bits & RegisterX64::INDEX_MASK),
    }
}
