use crate::records::native_module::NativeModule;

impl NativeModule {
    pub fn native_module_add_refs(&self, count: usize) -> usize {
        self.refcount
            .fetch_add(count, core::sync::atomic::Ordering::Relaxed)
            + count
    }
}
