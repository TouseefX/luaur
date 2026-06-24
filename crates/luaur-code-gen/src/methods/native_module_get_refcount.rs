impl crate::records::native_module::NativeModule {
    pub fn native_module_get_refcount(&self) -> usize {
        self.refcount.load(core::sync::atomic::Ordering::Relaxed)
    }
}
