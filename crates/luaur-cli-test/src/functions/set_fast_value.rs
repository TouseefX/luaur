//! Faithful port of `setFastValue<T>` from `luau/tests/main.cpp:316`:
//!
//! ```c++
//!     template<typename T>
//!     static void setFastValue(const std::string& name, T value)
//!     {
//!         for (Luau::FValue<T>* fvalue = Luau::FValue<T>::list; fvalue; fvalue = fvalue->next)
//!             if (fvalue->name == name)
//!                 fvalue->value = value;
//!     }
//! ```
//!
//! In Rust the per-`T` intrusive list head is supplied by `FValueList` (the
//! template's `static FValue<T>* list`), and the public `name`/`next` fields are
//! `pub(crate)` to luau-common, so we read them through a `#[repr(C)]` layout
//! mirror — the same technique `luau-cli-lib`'s `set_luau_flag` uses. The match
//! is name-equality only, exactly as the C++ template, and every match is set
//! (C++ does not `break` after the first hit).

use core::ffi::c_char;
use core::sync::atomic::Ordering;

use luaur_common::records::f_value::{FValue, FValueList};

/// `#[repr(C)]` mirror of `luaur_common::records::f_value::FValue<T>`, used to
/// read the otherwise crate-private `name` / `next` fields. Field order and
/// types must match `FValue<T>` exactly.
#[repr(C)]
struct FValueLayout<T> {
    value: core::cell::UnsafeCell<T>,
    dynamic: bool,
    name: *const c_char,
    next: core::cell::UnsafeCell<*const FValue<T>>,
    version: core::ffi::c_uint,
}

/// The set of `FValue` value types this harness instantiates (`bool`, `i32`),
/// mirroring the C++ `setFastValue<T>` template instantiations. `'static`
/// matches the global flag registry (`FValueList::head` is a `'static` list).
pub trait SetFastValueTarget: Copy + FValueList + 'static {}
impl SetFastValueTarget for bool {}
impl SetFastValueTarget for i32 {}

pub fn set_fast_value<T: SetFastValueTarget>(name: &str, value: T) {
    unsafe {
        let mut flag_ptr = <T as FValueList>::head().load(Ordering::Relaxed) as *const FValue<T>;

        while !flag_ptr.is_null() {
            let flag: &FValue<T> = &*flag_ptr;
            let layout = &*(flag_ptr as *const FValueLayout<T>);

            if !layout.name.is_null() {
                let flag_name = core::ffi::CStr::from_ptr(layout.name).to_bytes();
                if name.as_bytes() == flag_name {
                    flag.set(value);
                }
            }

            flag_ptr = *layout.next.get();
        }
    }
}
