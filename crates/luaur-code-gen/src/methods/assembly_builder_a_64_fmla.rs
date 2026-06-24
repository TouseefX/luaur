use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn fmla(&mut self, dst: RegisterA64, src1: RegisterA64, src2: RegisterA64) {
        // There is no scalar version of FMLA instruction
        // Vector instruction is used for both cases with proper sz bit.

        //                Q U        Sz  Rm    Opcode Rn    Rd
        let op: u32 = 0b0_0_0_011100_0_1_00000_110011_00000_00000;
        let q_bit: u32 = 1 << 30;
        let sz_bit: u32 = 1 << 22;

        if dst.kind() == KindA64::d {
            assert!(src1.kind() == KindA64::d && src2.kind() == KindA64::d);

            if self.log_text {
                self.log_append(format_args!(
                    " {:<12}sd{},d{},d{}\n",
                    "fmla",
                    dst.index(),
                    src1.index(),
                    src2.index()
                ));
            }

            self.place(
                (dst.index() as u32)
                    | ((src1.index() as u32) << 5)
                    | ((src2.index() as u32) << 16)
                    | op
                    | q_bit
                    | sz_bit,
            );
        } else if dst.kind() == KindA64::s {
            assert!(src1.kind() == KindA64::s && src2.kind() == KindA64::s);

            if self.log_text {
                self.log_append(format_args!(
                    " {:<12}ss{},s{},s{}\n",
                    "fmla",
                    dst.index(),
                    src1.index(),
                    src2.index()
                ));
            }

            self.place(
                (dst.index() as u32)
                    | ((src1.index() as u32) << 5)
                    | ((src2.index() as u32) << 16)
                    | op,
            );
        } else {
            assert!(
                dst.kind() == KindA64::q && src1.kind() == KindA64::q && src2.kind() == KindA64::q
            );

            if self.log_text {
                self.log_append(format_args!(
                    " {:<12}sv{}.4s,v{}.4s,v{}.4s\n",
                    "fmla",
                    dst.index(),
                    src1.index(),
                    src2.index()
                ));
            }

            self.place(
                (dst.index() as u32)
                    | ((src1.index() as u32) << 5)
                    | ((src2.index() as u32) << 16)
                    | op
                    | q_bit,
            );
        }

        self.commit();
    }
}
