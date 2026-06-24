use crate::enums::features_a_64::FeaturesA64;

pub fn get_cpu_features_a_64() -> u32 {
    let mut result: u32 = 0;

    #[cfg(target_os = "macos")]
    {
        use core::mem;
        use core::ptr;

        extern "C" {
            fn sysctlbyname(
                name: *const core::ffi::c_char,
                oldp: *mut core::ffi::c_void,
                oldlenp: *mut usize,
                newp: *mut core::ffi::c_void,
                newlen: usize,
            ) -> core::ffi::c_int;
        }

        unsafe {
            let mut jscvt: core::ffi::c_int = 0;
            let mut jscvt_len = mem::size_of_val(&jscvt);
            if sysctlbyname(
                c"hw.optional.arm.FEAT_JSCVT".as_ptr(),
                &mut jscvt as *mut _ as *mut core::ffi::c_void,
                &mut jscvt_len,
                ptr::null_mut(),
                0,
            ) == 0
                && jscvt == 1
            {
                result |= FeaturesA64::Feature_JSCVT as u32;
            }

            let mut adv_simd: core::ffi::c_int = 0;
            let mut adv_simd_len = mem::size_of_val(&adv_simd);
            if sysctlbyname(
                c"hw.optional.arm.AdvSIMD".as_ptr(),
                &mut adv_simd as *mut _ as *mut core::ffi::c_void,
                &mut adv_simd_len,
                ptr::null_mut(),
                0,
            ) == 0
                && adv_simd == 1
            {
                result |= FeaturesA64::Feature_AdvSIMD as u32;
            }
        }
    }

    result
}
