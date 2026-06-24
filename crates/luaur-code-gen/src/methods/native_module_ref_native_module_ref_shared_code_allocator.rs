use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
    pub fn native_module_ref(&mut self) {
        self.native_module = core::ptr::null();
    }
}
