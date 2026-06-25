//! Faithful port of `setFastFlags` from `luau/tests/main.cpp:324`:
//!
//! ```c++
//!     static void setFastFlags(const std::vector<doctest::String>& flags)
//!     {
//!         for (const doctest::String& flag : flags)
//!         {
//!             std::string_view view = flag.c_str();
//!             if (view == "true" || view == "false")
//!             {
//!                 for (Luau::FValue<bool>* flag = Luau::FValue<bool>::list; flag; flag = flag->next)
//!                     if (!skipFastFlag(flag->name))
//!                         flag->value = view == "true";
//!                 continue;
//!             }
//!             if (view.size() >= 2 && view[0] == 'D' && view[1] == 'F')
//!                 view.remove_prefix(1);
//!             if (view.substr(0, 4) == "FInt")
//!             {
//!                 auto [name, value] = parseFInt(view.substr(4));
//!                 setFastValue(name, value);
//!             }
//!             else
//!             {
//!                 auto [name, value] = parseFFlag(view.substr(0, 5) == "FFlag" ? view.substr(5) : view);
//!                 setFastValue(name, value);
//!             }
//!         }
//!     }
//! ```

use alloc::string::String;
use alloc::vec::Vec;

use core::ffi::c_char;
use core::sync::atomic::Ordering;

use luaur_common::records::f_value::{FValue, FValueList};

use crate::functions::parse_f_flag::parse_f_flag;
use crate::functions::parse_f_int::parse_f_int;
use crate::functions::set_fast_value::set_fast_value;
use crate::functions::skip_fast_flag::skip_fast_flag;

/// `#[repr(C)]` mirror of `luaur_common::records::f_value::FValue<T>` (see
/// `set_fast_value.rs`) used to read the crate-private `name` / `next` fields of
/// the bool flag list in the `"true"`/`"false"` default-on branch.
#[repr(C)]
struct FValueLayout<T> {
    value: core::cell::UnsafeCell<T>,
    dynamic: bool,
    name: *const c_char,
    next: core::cell::UnsafeCell<*const FValue<T>>,
    version: core::ffi::c_uint,
}

pub fn set_fast_flags(flags: Vec<String>) {
    for flag in &flags {
        let mut view: &str = flag.as_str();

        if view == "true" || view == "false" {
            let state = view == "true";
            unsafe {
                let mut p =
                    <bool as FValueList>::head().load(Ordering::Relaxed) as *const FValue<bool>;
                while !p.is_null() {
                    let f: &FValue<bool> = &*p;
                    let layout = &*(p as *const FValueLayout<bool>);

                    if !skip_fast_flag(layout.name) {
                        f.set(state);
                    }

                    p = *layout.next.get();
                }
            }
            continue;
        }

        // `DFInt`/`DFFlag` dynamic prefix: drop the leading 'D'.
        if view.len() >= 2 && view.as_bytes()[0] == b'D' && view.as_bytes()[1] == b'F' {
            view = &view[1..];
        }

        if view.len() >= 4 && &view[0..4] == "FInt" {
            let (name, value) = parse_f_int(&view[4..]);
            set_fast_value(name.as_str(), value);
        } else {
            // We want to prevent the footgun where '--fflags=LuauSomeFlag' is
            // ignored. We'll assume that this was declared as FFlag.
            let rest = if view.len() >= 5 && &view[0..5] == "FFlag" {
                &view[5..]
            } else {
                view
            };
            let (name, value) = parse_f_flag(rest);
            set_fast_value(name.as_str(), value);
        }
    }
}
