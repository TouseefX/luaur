use core::mem::offset_of;

use luaur_common::enums::luau_builtin_function::*;

use crate::records::native_context::NativeContext;

pub fn get_native_context_offset(bfid: i32) -> u32 {
    match bfid {
        LBF_MATH_ACOS => offset_of!(NativeContext, libm_acos) as u32,
        LBF_MATH_ASIN => offset_of!(NativeContext, libm_asin) as u32,
        LBF_MATH_ATAN2 => offset_of!(NativeContext, libm_atan2) as u32,
        LBF_MATH_ATAN => offset_of!(NativeContext, libm_atan) as u32,
        LBF_MATH_COSH => offset_of!(NativeContext, libm_cosh) as u32,
        LBF_MATH_COS => offset_of!(NativeContext, libm_cos) as u32,
        LBF_MATH_EXP => offset_of!(NativeContext, libm_exp) as u32,
        LBF_MATH_LOG10 => offset_of!(NativeContext, libm_log10) as u32,
        LBF_MATH_LOG => offset_of!(NativeContext, libm_log) as u32,
        LBF_MATH_SINH => offset_of!(NativeContext, libm_sinh) as u32,
        LBF_MATH_SIN => offset_of!(NativeContext, libm_sin) as u32,
        LBF_MATH_TANH => offset_of!(NativeContext, libm_tanh) as u32,
        LBF_MATH_TAN => offset_of!(NativeContext, libm_tan) as u32,
        LBF_MATH_FMOD => offset_of!(NativeContext, libm_fmod) as u32,
        LBF_MATH_POW => offset_of!(NativeContext, libm_pow) as u32,
        LBF_IR_MATH_LOG2 => offset_of!(NativeContext, libm_log2) as u32,
        LBF_MATH_LDEXP => offset_of!(NativeContext, libm_ldexp) as u32,
        _ => {
            debug_assert!(false, "Unsupported bfid");
            0
        }
    }
}
