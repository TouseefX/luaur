use crate::records::native_module::NativeModule;
use luaur_common::FFlag::LuauCodegenFreeBlocks;

impl NativeModule {
    pub fn native_module_get_module_base_address(&self) -> *const u8 {
        if LuauCodegenFreeBlocks.get() {
            self.code_allocation_data.code_start
        } else {
            self.module_base_address_deprecated
        }
    }
}
