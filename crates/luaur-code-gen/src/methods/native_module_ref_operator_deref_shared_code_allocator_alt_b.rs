use crate::records::native_module::NativeModule;

impl crate::records::native_module_ref::NativeModuleRef {
    pub fn native_module_ref_operator_deref(&self) -> &NativeModule {
        unsafe { &*self.native_module }
    }
}
