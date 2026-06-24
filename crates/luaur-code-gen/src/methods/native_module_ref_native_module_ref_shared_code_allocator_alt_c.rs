use crate::records::native_module::NativeModule;
use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
    #[inline]
    pub unsafe fn native_module_ref_native_module_ref_alt_c(&mut self, other: &NativeModuleRef) {
        self.native_module = other.native_module;
        if !self.native_module.is_null() {
            (*self.native_module).native_module_add_ref();
        }
    }
}
