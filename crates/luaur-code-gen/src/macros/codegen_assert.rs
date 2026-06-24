//! Node: `cxx:Macro:Luau.CodeGen:CodeGen/include/Luau/CodeGenCommon.h:CODEGEN_ASSERT`
//! (hand-fixed: the original translation passed `&str` where the handler
//! takes `*const c_char` and could never have expanded; mirrors LUAU_ASSERT!)

#[macro_export]
macro_rules! CODEGEN_ASSERT {
    ($expr:expr) => {
        if !($expr) {
            if luaur_common::assert_call_handler(
                concat!(stringify!($expr), "\0").as_ptr() as *const core::ffi::c_char,
                concat!(file!(), "\0").as_ptr() as *const core::ffi::c_char,
                line!() as i32,
                c"unknown".as_ptr(),
            ) != 0
            {
                luaur_common::LUAU_DEBUGBREAK!();
            }
        }
    };
    // Tolerant 2-arg form: the C++ `CODEGEN_ASSERT(cond && "message")` idiom
    // routinely lands as `CODEGEN_ASSERT!(cond, "message")`. Accept it (mirrors
    // the LUAU_ASSERT! tolerance) instead of failing on the separator.
    ($expr:expr, $msg:expr) => {
        if !($expr) {
            if luaur_common::assert_call_handler(
                concat!(stringify!($expr), " : ", stringify!($msg), "\0").as_ptr()
                    as *const core::ffi::c_char,
                concat!(file!(), "\0").as_ptr() as *const core::ffi::c_char,
                line!() as i32,
                c"unknown".as_ptr(),
            ) != 0
            {
                luaur_common::LUAU_DEBUGBREAK!();
            }
        }
    };
}

pub use CODEGEN_ASSERT;
