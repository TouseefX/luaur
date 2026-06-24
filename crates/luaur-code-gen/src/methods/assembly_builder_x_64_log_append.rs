use crate::records::assembly_builder_x_64::AssemblyBuilderX64;

impl AssemblyBuilderX64 {
    pub fn log_append(&mut self, args: core::fmt::Arguments<'_>) {
        use core::fmt::Write;
        let _ = core::fmt::write(&mut self.text, args);
    }
}
