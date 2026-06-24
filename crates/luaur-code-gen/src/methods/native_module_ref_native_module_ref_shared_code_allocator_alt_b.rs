use crate::records::native_module::NativeModule;
use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
    #[inline]
    pub unsafe fn native_module_ref_native_module_assignment(
        &mut self,
        native_module: *const NativeModule,
    ) {
        self.native_module = native_module;
        if !native_module.is_null() {
            (*native_module).native_module_add_ref();
        }
    }
}
