use crate::records::base_code_gen_context::BaseCodeGenContext;
use luaur_common::FFlag;

impl BaseCodeGenContext {
    pub fn drop(&mut self) {
        if FFlag::LuauCodegenFreeBlocks.get() {
            self.code_allocator.deallocate(self.gate_allocation_data);
        }
    }
}
