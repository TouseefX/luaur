use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fcvt(&mut self, dst: RegisterA64, src: RegisterA64) {
        if dst.kind() == crate::enums::kind_a_64::KindA64::s
            && src.kind() == crate::enums::kind_a_64::KindA64::d
        {
            self.place_r_1(
                b"fcvt\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b11110_01_1_0001_00_10000,
            );
        } else if dst.kind() == crate::enums::kind_a_64::KindA64::d
            && src.kind() == crate::enums::kind_a_64::KindA64::s
        {
            self.place_r_1(
                b"fcvt\0".as_ptr() as *const core::ffi::c_char,
                dst,
                src,
                0b11110_00_1_0001_01_10000,
            );
        } else {
            luaur_common::LUAU_ASSERT!(false);
        }
    }
}
