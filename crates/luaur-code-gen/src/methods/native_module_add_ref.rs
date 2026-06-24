use crate::records::native_module::NativeModule;

impl NativeModule {
    pub fn native_module_add_ref(&self) -> usize {
        self.refcount
            .fetch_add(1, core::sync::atomic::Ordering::Relaxed)
            + 1
    }
}
