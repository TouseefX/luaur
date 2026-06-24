use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocator::CodeAllocator;
use core::ffi::{c_int, c_void};

#[cfg(not(target_os = "windows"))]
extern "C" {
    fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
}

pub fn allocate_pages_impl(size: usize) -> *mut u8 {
    CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

    #[cfg(target_os = "windows")]
    {
        crate::functions::allocate_pages_impl_code_allocator::allocate_pages_impl(size)
    }

    #[cfg(not(target_os = "windows"))]
    unsafe {
        const PROT_READ: c_int = 0x1;
        const PROT_WRITE: c_int = 0x2;
        const MAP_PRIVATE: c_int = 0x02;
        #[cfg(any(target_os = "linux", target_os = "android"))]
        const MAP_ANON: c_int = 0x20;
        #[cfg(any(target_os = "macos", target_os = "ios", target_os = "freebsd"))]
        const MAP_ANON: c_int = 0x1000;
        #[cfg(target_os = "macos")]
        const MAP_JIT: c_int = 0x0800;
        #[cfg(not(target_os = "macos"))]
        const MAP_JIT: c_int = 0;

        let result = mmap(
            core::ptr::null_mut(),
            size,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANON | MAP_JIT,
            -1,
            0,
        );

        if result as isize == -1 {
            core::ptr::null_mut()
        } else {
            result.cast()
        }
    }
}
