#[allow(non_snake_case)]
pub fn flush_instruction_cache_mut(mem: *mut u8, size: usize) {
    #[cfg(target_arch = "wasm32")]
    {
        // No-op for Emscripten/Wasm
    }
    #[cfg(all(not(target_arch = "wasm32"), target_vendor = "apple"))]
    {
        extern "C" {
            fn sys_icache_invalidate(start: *mut core::ffi::c_void, len: usize);
        }
        unsafe {
            sys_icache_invalidate(mem as *mut core::ffi::c_void, size);
        }
    }
    #[cfg(all(not(target_arch = "wasm32"), not(target_vendor = "apple")))]
    {
        // Note: __builtin___clear_cache is a GCC/Clang intrinsic.
        // In Rust, we use the llvm.clear_cache intrinsic via core::arch.
        extern "C" {
            #[link_name = "llvm.clear_cache"]
            fn llvm_clear_cache(begin: *mut i8, end: *mut i8);
        }
        unsafe {
            llvm_clear_cache(mem as *mut i8, mem.add(size) as *mut i8);
        }
    }
}
