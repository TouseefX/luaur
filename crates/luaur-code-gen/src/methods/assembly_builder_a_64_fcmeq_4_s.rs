use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fcmeq_4s(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
        if self.log_text {
            self.log_append(format_args!(
                " {:<12}v{}.4s,v{}.4s,v{}.4s\n",
                "fcmeq",
                dst.index(),
                src1.index(),
                src2.index()
            ));
        }

        let op: u32 = 0b0_1_0_01110001_00000_111001_00000_00000;

        self.place(
            (dst.index() as u32)
                | ((src1.index() as u32) << 5)
                | ((src2.index() as u32) << 16)
                | op,
        );

        self.commit();
    }
}
