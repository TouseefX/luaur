use crate::records::allocator::Allocator;
use crate::records::page::Page;
use alloc::alloc::{dealloc, Layout};

#[allow(non_snake_case)]
impl Allocator {
    /// Destructor logic for Allocator.
    /// Note: In Rust this is typically handled by a Drop implementation,
    /// but we provide the requested method for compatibility.
    pub fn allocator_allocator_dtor(&mut self) {
        unsafe {
            let mut page = self.root;
            while !page.is_null() {
                let next = (*page).next;

                // We must calculate the layout used during allocation.
                // The C++ code uses `operator delete(page)`, which assumes the size
                // matches the allocation. However, `allocate` can create larger pages.
                // This implementation assumes standard Page size for simplicity,
                // matching the C++ `operator delete(page)` behavior on a `Page*`.
                let layout = Layout::new::<Page>();
                dealloc(page as *mut u8, layout);

                page = next;
            }
            self.root = core::ptr::null_mut();
        }
    }
}
