use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn ins_4_s_register_a_64_register_a_64_u8(
        &mut self,
        dst: RegisterA64,
        src: RegisterA64,
        index: u8,
    ) {
        debug_assert!(
            dst.kind() == crate::enums::kind_a_64::KindA64::q
                && src.kind() == crate::enums::kind_a_64::KindA64::w
        );
        debug_assert!(index < 4);

        if self.log_text {
            self.log_append(format_args!(
                " {:<12}v{}.s[{}],w{}\n",
                "ins",
                dst.index(),
                index,
                src.index()
            ));
        }

        let op: u32 = 0b0_1_0_01110000_00100_0_0011_1;

        self.place(
            dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19,
        );
        self.commit();
    }
}
