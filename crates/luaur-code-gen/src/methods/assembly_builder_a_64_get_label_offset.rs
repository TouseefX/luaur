use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::label::Label;

impl crate::records::assembly_builder_a_64::AssemblyBuilderA64 {
    pub fn get_label_offset(&self, label: &Label) -> u32 {
        CODEGEN_ASSERT!(label.location != !0u32);
        label.location * 4
    }
}
