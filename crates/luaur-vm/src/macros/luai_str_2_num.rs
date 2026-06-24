#[macro_export]
#[allow(non_snake_case)]
macro_rules! luai_str2num {
    ($s:expr, $p:expr) => {
        unsafe { crate::macros::luai_str_2_num::strtod($s, $p) }
    };
}

#[cfg(not(target_arch = "wasm32"))]
extern "C" {
    pub fn strtod(s: *const core::ffi::c_char, endptr: *mut *mut core::ffi::c_char) -> f64;
}

#[cfg(target_arch = "wasm32")]
#[inline]
pub fn strtod(_s: *const core::ffi::c_char, _endptr: *mut *mut core::ffi::c_char) -> f64 {
    0.0
}

pub use luai_str2num;
