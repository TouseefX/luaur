#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline]
#[allow(non_snake_case)]
pub fn roundsd_sse41<const ROUNDING: i32>(v: f64) -> f64 {
    #[cfg(target_arch = "x86_64")]
    {
        // The C++ code uses _MM_FROUND_NO_EXC which is 0x08.
        // In Rust core::arch::x86_64, this is typically passed as an immediate to _mm_round_sd.
        // We must ensure the target_feature is available or this is only called when SSE4.1 is supported.
        unsafe {
            let av = _mm_set_sd(v);
            // _MM_FROUND_NO_EXC is 8.
            let rv = _mm_round_sd(av, av, ROUNDING | 8);
            _mm_cvtsd_f64(rv)
        }
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        // This function is guarded by LUAU_TARGET_SSE41 in C++,
        // which implies it is only called on supported hardware.
        // For non-x86_64 platforms, this is unreachable or a stub.
        let _ = ROUNDING;
        v
    }
}
