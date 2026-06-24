use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocation_data::CodeAllocationData;
use crate::records::native_module::NativeModule;
use luaur_common::FFlag;

impl NativeModule {
    pub fn native_module_get_code_allocation_data(&self) -> CodeAllocationData {
        CODEGEN_ASSERT!(FFlag::LuauCodegenFreeBlocks.get());
        self.code_allocation_data
    }
}
