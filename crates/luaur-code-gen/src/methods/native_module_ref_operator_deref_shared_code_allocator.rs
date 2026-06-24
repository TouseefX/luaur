impl crate::records::native_module_ref::NativeModuleRef {
    pub fn native_module_ref_operator_arrow(
        &self,
    ) -> *const crate::records::native_module::NativeModule {
        self.native_module
    }
}
