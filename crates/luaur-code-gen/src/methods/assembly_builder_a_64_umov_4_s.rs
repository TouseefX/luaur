use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn umov_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
        CODEGEN_ASSERT!(dst.kind() == KindA64::w);
        CODEGEN_ASSERT!(src.kind() == KindA64::q);
        CODEGEN_ASSERT!(index < 4);

        if self.log_text {
            self.log_append(format_args!(
                " {:<12}w{},v{}.s[{}]\n",
                "umov",
                dst.index(),
                src.index(),
                index
            ));
        }

        let op: u32 = 0b0_0_0_01110000_00100_001111_00000_00000;

        self.place(dst.index() as u32 | (src.index() as u32) << 5 | op | (index as u32) << 19);

        self.commit();
    }
}
