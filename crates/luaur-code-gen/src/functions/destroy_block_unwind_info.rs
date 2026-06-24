use crate::functions::visit_fde_entries::visit_fde_entries;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::macros::codegen_target_a_64::CODEGEN_TARGET_A64;
use crate::macros::codegen_target_x_64::CODEGEN_TARGET_X64;

#[cfg(target_os = "windows")]
extern "system" {
    fn RtlDeleteFunctionTable(function_table: *mut core::ffi::c_void) -> i32;
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
extern "C" {
    fn __deregister_frame(begin: *const core::ffi::c_void);
}

pub unsafe extern "C" fn destroy_block_unwind_info(
    context: *mut core::ffi::c_void,
    unwind_data: *mut core::ffi::c_void,
) {
    #[cfg(target_os = "windows")]
    {
        if CODEGEN_TARGET_X64 {
            let result = unsafe { RtlDeleteFunctionTable(unwind_data) };
            if result == 0 {
                CODEGEN_ASSERT!(false);
            }
        }
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    {
        if CODEGEN_TARGET_X64 || CODEGEN_TARGET_A64 {
            unsafe {
                visit_fde_entries(unwind_data as *mut core::ffi::c_char, __deregister_frame);
            }
        }
    }
}
