use crate::enums::address_kind_a_64::AddressKindA64;
use crate::records::address_a_64::AddressA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn place_p(
        &mut self,
        name: *const core::ffi::c_char,
        src1: RegisterA64,
        src2: RegisterA64,
        dst: AddressA64,
        op: u8,
        opc: u8,
        sizelog: i32,
    ) {
        if self.log_text {
            self.log_c_char_register_a_64_register_a_64_address_a_64(name, src1, src2, dst);
        }

        assert!(dst.kind == AddressKindA64::imm);
        assert!(dst.data >= (-128 * (1i32 << sizelog)) && dst.data <= (127 * (1i32 << sizelog)));
        assert!(dst.data % (1i32 << sizelog) == 0);

        self.place(
            (src1.index() as u32)
                | ((dst.base.index() as u32) << 5)
                | ((src2.index() as u32) << 10)
                | (((dst.data >> sizelog) as u32 & 127) << 15)
                | ((op as u32) << 22)
                | ((opc as u32) << 30),
        );
        self.commit();
    }
}
