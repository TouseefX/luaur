use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn sub_register_a_64_register_a_64_register_a_64_i32(
        &mut self,
        dst: RegisterA64,
        src1: RegisterA64,
        src2: RegisterA64,
        shift: i32,
    ) {
        if src1.kind() == KindA64::x && src2.kind() == KindA64::w {
            self.place_e_r(c"sub".as_ptr(), dst, src1, src2, 0b10_01011, shift);
        } else {
            self.place_sr_3(c"sub".as_ptr(), dst, src1, src2, 0b10_01011, shift, 0);
        }
    }
}
