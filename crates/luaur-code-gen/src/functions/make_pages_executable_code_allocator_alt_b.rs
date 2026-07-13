use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::code_allocator::CodeAllocator;

#[allow(non_snake_case)]
pub unsafe fn make_pages_executable_mut(mem: *mut u8, size: usize) -> bool {
    CODEGEN_ASSERT!(CodeAllocator::align_to_page_size(mem as usize) == mem as usize);
    CODEGEN_ASSERT!(size == CodeAllocator::align_to_page_size(size));

    // Android/bionic exposes the same mprotect(2) as Linux. This never requests
    // PROT_WRITE|PROT_EXEC together (only ever flips RW <-> RX), which is the
    // flip-flop pattern Android's W^X enforcement (SELinux + newer API-level
    // hardening) expects from a JIT; still worth an on-device smoke test since
    // OEM/hardened builds can vary.
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        use core::ffi::c_int;
        use core::ffi::c_void;

        extern "C" {
            fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
        }

        const PROT_READ: c_int = 0x1;
        const PROT_EXEC: c_int = 0x4;

        mprotect(mem as *mut c_void, size, PROT_READ | PROT_EXEC) == 0
    }
    #[cfg(target_os = "macos")]
    {
        use core::ffi::c_int;
        use core::ffi::c_void;

        extern "C" {
            fn mprotect(addr: *mut c_void, len: usize, prot: c_int) -> c_int;
        }

        const PROT_READ: c_int = 0x1;
        const PROT_EXEC: c_int = 0x4;

        mprotect(mem as *mut c_void, size, PROT_READ | PROT_EXEC) == 0
    }
    #[cfg(target_os = "windows")]
    {
        use core::ffi::c_void;
        use windows_sys::Win32::System::Memory::{VirtualProtect, PAGE_EXECUTE_READ};

        let mut old_protect: u32 = 0;
        VirtualProtect(
            mem as *const c_void,
            size,
            PAGE_EXECUTE_READ,
            &mut old_protect as *mut u32,
        ) != 0
    }
    #[cfg(not(any(
        target_os = "linux",
        target_os = "macos",
        target_os = "android",
        target_os = "windows"
    )))]
    {
        false
    }
}
