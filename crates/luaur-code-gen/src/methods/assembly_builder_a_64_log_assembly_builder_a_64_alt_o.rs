use crate::records::address_a_64::AddressA64;

impl crate::records::assembly_builder_a_64::AssemblyBuilderA64 {
    pub fn log_address_a_64(&mut self, addr: AddressA64) {
        self.text.push('[' as u8 as char);
        match addr.kind {
            crate::enums::address_kind_a_64::AddressKindA64::reg => {
                self.log_register_a_64(addr.base);
                self.text.push(',' as u8 as char);
                self.log_register_a_64(addr.offset);
            }
            crate::enums::address_kind_a_64::AddressKindA64::imm => {
                self.log_register_a_64(addr.base);
                if addr.data != 0 {
                    self.log_append(format_args!(",#{}", addr.data));
                }
            }
            crate::enums::address_kind_a_64::AddressKindA64::pre => {
                self.log_register_a_64(addr.base);
                if addr.data != 0 {
                    self.log_append(format_args!(",#{}", addr.data));
                }
                self.text.push(']' as u8 as char);
                self.text.push('!' as u8 as char);
                return;
            }
            crate::enums::address_kind_a_64::AddressKindA64::post => {
                self.log_register_a_64(addr.base);
                self.text.push(']' as u8 as char);
                self.text.push('!' as u8 as char);
                if addr.data != 0 {
                    self.log_append(format_args!(",#{}", addr.data));
                }
                return;
            }
        }
        self.text.push(']' as u8 as char);
    }
}
