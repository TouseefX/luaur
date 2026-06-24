use crate::records::native_module_ref::NativeModuleRef;

impl NativeModuleRef {
    pub fn native_module_ref_swap(&mut self, other: &mut NativeModuleRef) {
        let temp = self.native_module;
        self.native_module = other.native_module;
        other.native_module = temp;
    }
}
