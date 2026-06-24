use core::ffi::c_char;
use core::sync::atomic::Ordering;

use luaur_common::records::f_value::{FValue, FValueList};

pub fn set_luau_flags_bool(state: bool) {
    unsafe {
        let mut flag_ptr = <bool as FValueList>::head().load(Ordering::Relaxed);

        while !flag_ptr.is_null() {
            let flag = &*flag_ptr;

            let name_ptr = flag_ptr.cast::<FValueLayout<bool>>().read().name;

            if !name_ptr.is_null() {
                let name_cstr = core::ffi::CStr::from_ptr(name_ptr);
                let name_bytes = name_cstr.to_bytes();

                if name_bytes.starts_with(b"Luau") {
                    flag.set(state);
                }
            }

            flag_ptr = flag_ptr.cast::<FValueLayout<bool>>().read().next as *mut FValue<bool>;
        }
    }
}

#[repr(C)]
struct FValueLayout<T> {
    value: core::cell::UnsafeCell<T>,
    dynamic: bool,
    name: *const c_char,
    next: *const FValue<T>,
    version: core::ffi::c_uint,
}
