use core::ffi::c_char;
use core::sync::atomic::Ordering;

use luaur_common::functions::is_analysis_flag_experimental::isAnalysisFlagExperimental;
use luaur_common::records::f_value::{FValue, FValueList};

pub fn set_luau_flag(name: &str, state: bool) {
    let name_ptr = name.as_ptr() as *const c_char;

    unsafe {
        let mut flag_ptr = <bool as FValueList>::head().load(Ordering::Relaxed);

        #[repr(C)]
        struct FValueLayout<T> {
            value: core::cell::UnsafeCell<T>,
            dynamic: bool,
            name: *const c_char,
            next: core::cell::UnsafeCell<*const FValue<T>>,
            version: core::ffi::c_uint,
        }

        while !flag_ptr.is_null() {
            let flag: &FValue<bool> = &*flag_ptr;

            let layout = &*(flag_ptr as *const FValueLayout<bool>);
            let flag_name_ptr = layout.name;

            if !flag_name_ptr.is_null() {
                let flag_name_bytes = core::ffi::CStr::from_ptr(flag_name_ptr).to_bytes();

                if name.as_bytes() == flag_name_bytes {
                    flag.set(state);
                    return;
                }

                let version = flag.version();
                if version != 0 {
                    // Compare against "<flag_name><version_decimal>" without allocations.
                    let version_bytes = version.to_string();

                    if name.as_bytes().len() == flag_name_bytes.len() + version_bytes.len()
                        && name.as_bytes()[..flag_name_bytes.len()] == *flag_name_bytes
                        && name.as_bytes()[flag_name_bytes.len()..] == *version_bytes.as_bytes()
                    {
                        flag.set(state);
                        return;
                    }
                }
            }

            flag_ptr = *layout.next.get() as *mut FValue<bool>;
        }

        // Match original behavior: only warn for flags that look like Luau flags, and skip experimental flags.
        if name.as_bytes().starts_with(b"Luau") && !isAnalysisFlagExperimental(name_ptr) {
            eprintln!("Warning: unrecognized flag '{}'.", name);
        }
    }
}
