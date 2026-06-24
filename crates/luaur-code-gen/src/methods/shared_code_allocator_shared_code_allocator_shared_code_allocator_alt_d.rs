use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::shared_code_allocator::SharedCodeAllocator;
use core::sync::atomic::Ordering;

impl SharedCodeAllocator {
    pub fn drop(&mut self) {
        CODEGEN_ASSERT!(self.identified_modules.is_empty());
        CODEGEN_ASSERT!(self.anonymous_module_count.load(Ordering::Relaxed) == 0);
    }
}
