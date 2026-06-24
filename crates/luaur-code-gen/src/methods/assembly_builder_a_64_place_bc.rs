use crate::enums::kind::Kind;
use crate::records::assembly_builder_a_64::AssemblyBuilderA64;
use crate::records::label::Label;
use crate::records::patch::Patch;

impl AssemblyBuilderA64 {
    pub fn place_bc(
        &mut self,
        name: *const core::ffi::c_char,
        label: &mut Label,
        op: u8,
        cond: u8,
    ) {
        self.place(cond as u32 | ((op as u32) << 24));
        self.commit();

        self.patch_label(label, Kind::Imm19);

        if self.log_text {
            self.log_c_char_label(name, *label);
        }
    }
}
