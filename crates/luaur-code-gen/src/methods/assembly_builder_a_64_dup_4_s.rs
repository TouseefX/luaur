use crate::enums::kind_a_64::KindA64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn dup_4s(&mut self, dst: RegisterA64, src: RegisterA64, index: u8) {
        if dst.kind() == KindA64::s {
            CODEGEN_ASSERT!(src.kind() == KindA64::q);
            CODEGEN_ASSERT!(index < 4);

            if self.log_text {
                self.log_append(format_args!(
                    " {:<12}s{},v{}.s[{}]\n",
                    "dup",
                    dst.index(),
                    src.index(),
                    index
                ));
            }

            let op: u32 = 0b01_0_11110000_00100_0_0000_1;
            self.place(
                dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19,
            );
        } else {
            CODEGEN_ASSERT!(src.kind() == KindA64::q);
            CODEGEN_ASSERT!(index < 4);

            if self.log_text {
                self.log_append(format_args!(
                    " {:<12}v{}.4s,v{}.s[{}]\n",
                    "dup",
                    dst.index(),
                    src.index(),
                    index
                ));
            }

            let op: u32 = 0b010_01110000_00100_0_0000_1;
            self.place(
                dst.index() as u32 | (src.index() as u32) << 5 | op << 10 | (index as u32) << 19,
            );
        }

        self.commit();
    }
}
