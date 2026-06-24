use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn tst_register_a_64_register_a_64_i32(
        &mut self,
        src1: RegisterA64,
        src2: RegisterA64,
        shift: i32,
    ) {
        let dst = if src1.kind() == crate::enums::kind_a_64::KindA64::x {
            RegisterA64::xzr
        } else {
            RegisterA64::wzr
        };

        self.place_sr_3(c"tst".as_ptr(), dst, src1, src2, 0b11_01010, shift, 0);
    }
}
