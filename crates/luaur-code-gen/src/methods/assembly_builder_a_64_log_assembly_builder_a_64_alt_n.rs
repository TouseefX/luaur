use crate::enums::kind_a_64::KindA64;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::register_a_64::RegisterA64;

impl AssemblyBuilderA64 {
    pub fn log_register_a_64(&mut self, reg: RegisterA64) {
        match reg.kind() {
            KindA64::w => {
                if reg.index() == 31 {
                    self.text.push_str("wzr");
                } else {
                    self.log_append(format_args!("w{}", reg.index()));
                }
            }
            KindA64::x => {
                if reg.index() == 31 {
                    self.text.push_str("xzr");
                } else {
                    self.log_append(format_args!("x{}", reg.index()));
                }
            }
            KindA64::s => {
                self.log_append(format_args!("s{}", reg.index()));
            }
            KindA64::d => {
                self.log_append(format_args!("d{}", reg.index()));
            }
            KindA64::q => {
                self.log_append(format_args!("q{}", reg.index()));
            }
            KindA64::none => {
                if reg.index() == 31 {
                    self.text.push_str("sp");
                } else {
                    debug_assert!(false, "Unexpected register kind");
                }
            }
        }
    }
}
