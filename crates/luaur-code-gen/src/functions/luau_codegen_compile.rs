use crate::functions::compile_internal::compile_internal;
use crate::records::compilation_options::CompilationOptions;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::c_int;

pub fn luau_codegen_compile(L: *mut lua_State, idx: c_int) {
    unsafe {
        let _ = compile_internal(
            &None,
            L,
            idx,
            &CompilationOptions::default(),
            core::ptr::null_mut(),
        );
    }
}
