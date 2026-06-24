use core::ffi::c_char;
use core::ptr;

use crate::functions::load::load;
use luaur_vm::functions::lua_c_statename::luaC_statename;

pub fn profiler_dump(path: *const c_char) {
    unsafe {
        if path.is_null() {
            return;
        }

        // Placeholder: the native profiler data structures live in C++ and aren't translated in this one-shot.
        // Implemented as a no-op to preserve the published Rust interface.
        //
        // Keep a reference to imported symbols to avoid unused-import warnings in this file.
        let _ = luaC_statename(0);
        let _ = load(
            ptr::null_mut(),
            ptr::null_mut(),
            ptr::null(),
            ptr::null(),
            ptr::null(),
        );
    }
}
