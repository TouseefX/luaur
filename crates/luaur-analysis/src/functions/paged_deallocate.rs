use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

use crate::functions::paged_allocate::page_size;

#[cfg(not(target_os = "windows"))]
use core::ffi::{c_int, c_void};

#[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
extern "C" {
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
}

/// Port of `Luau::pagedDeallocate`.
///
/// Frees a block previously returned by `paged_allocate`. `size` is always the
/// block size the matching allocation was made with (`kBlockSizeBytes`), which
/// lets the default heap path reconstruct the same `Layout`.
pub fn paged_deallocate(ptr: *mut core::ffi::c_void, size: usize) {
    // By default we use operator new/delete instead of malloc/free so that they
    // can be overridden externally.
    if !FFlag::DebugLuauFreezeArena.get() {
        // `::operator delete(ptr)`. Reconstruct the exact `Layout` used by
        // `paged_allocate`'s default branch.
        if ptr.is_null() || size == 0 {
            return;
        }
        if let Ok(layout) = core::alloc::Layout::from_size_align(size, page_size()) {
            unsafe { alloc::alloc::dealloc(ptr as *mut u8, layout) };
        }
        return;
    }

    #[cfg(target_os = "windows")]
    {
        extern "C" {
            fn _aligned_free(ptr: *mut core::ffi::c_void);
        }
        unsafe { _aligned_free(ptr) };
    }

    #[cfg(target_os = "freebsd")]
    {
        extern "C" {
            fn free(ptr: *mut c_void);
        }
        unsafe { free(ptr) };
    }

    #[cfg(not(any(target_os = "windows", target_os = "freebsd")))]
    {
        let rc = unsafe { munmap(ptr as *mut c_void, size) };
        LUAU_ASSERT!(rc == 0);
    }
}
