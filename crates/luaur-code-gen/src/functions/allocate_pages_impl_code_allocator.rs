use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocator::CodeAllocator;

pub fn allocate_pages_impl(size: usize) -> *mut u8 {
    CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

    #[cfg(target_os = "windows")]
    {
        use core::ffi::c_void;
        use windows::Win32::System::Memory::{
            VirtualAlloc, MEM_COMMIT, MEM_RESERVE, PAGE_READWRITE,
        };

        unsafe { VirtualAlloc(None, size, MEM_RESERVE | MEM_COMMIT, PAGE_READWRITE) as *mut u8 }
    }

    #[cfg(not(target_os = "windows"))]
    {
        crate::functions::allocate_pages_impl_code_allocator_alt_b::allocate_pages_impl(size)
    }
}
