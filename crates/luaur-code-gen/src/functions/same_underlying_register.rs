use crate::enums::size_x_64::SizeX64;
use crate::records::register_x_64::RegisterX64;

pub fn same_underlying_register(a: RegisterX64, b: RegisterX64) -> bool {
    let underlying_size_a = if a.size() == SizeX64::xmmword {
        SizeX64::xmmword
    } else {
        SizeX64::qword
    };

    let underlying_size_b = if b.size() == SizeX64::xmmword {
        SizeX64::xmmword
    } else {
        SizeX64::qword
    };

    underlying_size_a == underlying_size_b && a.index() == b.index()
}
