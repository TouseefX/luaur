use crate::functions::normalize_state_offsets::normalize_state_offsets;
use crate::functions::strip_lines_containing::strip_lines_containing;
use crate::records::ir_assembly_fixture::IrAssemblyFixture;
use alloc::string::String;
use luaur_code_gen::functions::get_assembly_from_ir::get_assembly_from_ir;

impl IrAssemblyFixture {
    pub fn lower(&mut self) -> String {
        let mut text = unsafe {
            get_assembly_from_ir(&mut self.build, self.options.clone(), core::ptr::null_mut())
        };
        strip_lines_containing(&mut text, "; skipping ");
        normalize_state_offsets(&mut text);
        text
    }
}
