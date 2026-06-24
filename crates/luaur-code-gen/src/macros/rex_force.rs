use crate::enums::size_x_64::SizeX64;
use crate::records::register_x_64::RegisterX64;

#[allow(non_snake_case)]
pub const fn REX_FORCE(reg: RegisterX64) -> u8 {
    // Note: RegisterX64::size() is not const, but we can compute it from bits.
    // Based on RegisterX64's internal structure (bits), we extract the size.
    let size_bits = reg.bits & RegisterX64::SIZE_MASK;

    // SizeX64::byte is an enum variant. Since we cannot use == in const if the impl is not const,
    // and size() is not const, we compare the raw bits against the discriminant of SizeX64::byte.
    if size_bits == SizeX64::byte as u8 && reg.index() >= 4 {
        0x40
    } else {
        0x00
    }
}
