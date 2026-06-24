use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocator::CodeAllocator;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use core::ffi::c_int;

#[cfg(any(target_os = "linux", target_os = "macos"))]
use core::ffi::c_void;

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern "C" {
    fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
}

#[allow(non_snake_case)]
pub fn make_pages_not_executable_mut(mem: *mut u8, size: usize) -> bool {
    CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
    CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        const PROT_READ: c_int = 0x1;
        const PROT_WRITE: c_int = 0x2;

        unsafe { mprotect(mem as *mut c_void, size, PROT_READ | PROT_WRITE) == 0 }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        false
    }
}
