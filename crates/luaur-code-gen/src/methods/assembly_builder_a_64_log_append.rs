use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use core::fmt::Write;

impl AssemblyBuilderA64 {
    pub fn log_append(&mut self, args: core::fmt::Arguments<'_>) {
        let _ = core::fmt::write(&mut self.text, args);
    }
}
