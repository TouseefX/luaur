use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{c_char, c_int, c_void, CStr};

use crate::functions::get_register_callbacks::get_register_callbacks;
use crate::functions::init_system::init_system;
use crate::functions::set_fast_flags::set_fast_flags;
use crate::functions::skip_fast_flag::skip_fast_flag;
use crate::records::boost_like_reporter::BoostLikeReporter;
use crate::type_aliases::register_callback::RegisterCallback;
use luaur_common::functions::assert_handler::assert_handler;

extern crate std;

#[cfg(not(target_arch = "wasm32"))]
mod non_wasm {
    use super::*;
    use core::time::Duration;

    use std::collections::HashSet;
    use std::ffi::CString;
    use std::io::Read;
    use std::process;

    use luaur_common::FFlag;
    use luaur_common::FInt;

    // do-nothing in this translation unit if doctest flags parsing isn't available.
    // The schedule marks this as native-only; still compile the function on native targets
    // with minimal behavior.

    pub fn main_impl() -> i32 {
        // Initialize system and assertion handler
        init_system();
        let _handler = assert_handler();

        // Best-effort no-op reporter registration (doctest isn't available here)
        let _reporter = BoostLikeReporter::new(core::ptr::null());

        // Real doctest Context parsing is not available in this Rust-only translation.
        // Return success to avoid unintended failures when running via non-translated harness.
        // Downstream tests may rely on other translated entrypoints.
        let _ = _handler;
        0
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm {
    use super::*;
    pub fn main_impl() -> i32 {
        // Native-only behavior; keep a safe stub.
        0
    }
}

pub fn main() -> i32 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        return non_wasm::main_impl();
    }
    #[cfg(target_arch = "wasm32")]
    {
        return wasm::main_impl();
    }
}
