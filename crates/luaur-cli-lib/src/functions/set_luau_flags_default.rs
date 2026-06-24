use core::sync::atomic::Ordering;
use luaur_common::functions::is_analysis_flag_experimental::isAnalysisFlagExperimental;
use luaur_common::records::f_value::{FValue, FValueList};

pub fn set_luau_flags_default() {
    let mut flag_ptr = <bool as FValueList>::head().load(Ordering::Relaxed);

    while !flag_ptr.is_null() {
        let flag = unsafe { &*flag_ptr };

        // The FValue fields 'name' and 'next' are pub(crate) in luau-common, so they are private to luau-cli-lib.
        // However, the FValue struct is a #[repr(C)] struct. We can access the fields by casting to a local
        // shadow struct with the same layout or by using pointer offsets.
        // Given the FValue definition: { value: UnsafeCell<T>, dynamic: bool, name: *const c_char, next: UnsafeCell<*const FValue<T>>, ... }
        #[repr(C)]
        struct FValueLayout<T> {
            value: core::cell::UnsafeCell<T>,
            dynamic: bool,
            name: *const core::ffi::c_char,
            next: core::cell::UnsafeCell<*const FValue<T>>,
        }

        let layout = unsafe { &*(flag_ptr as *const FValueLayout<bool>) };
        let name_ptr = layout.name;

        if !name_ptr.is_null() {
            let name_cstr = unsafe { core::ffi::CStr::from_ptr(name_ptr) };
            let name_bytes = name_cstr.to_bytes();

            if name_bytes.starts_with(b"Luau") && !isAnalysisFlagExperimental(name_ptr) {
                flag.set(true);
            }
        }

        flag_ptr = unsafe { *layout.next.get() as *mut FValue<bool> };
    }
}
