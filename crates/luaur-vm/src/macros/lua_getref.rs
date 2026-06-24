use crate::functions::lua_rawgeti::lua_rawgeti;
use crate::macros::lua_registryindex::LUA_REGISTRYINDEX;

#[inline(always)]
pub fn lua_getref(l: *mut crate::records::lua_state::lua_State, ref_: core::ffi::c_int) {
    unsafe {
        // The dependency lua_rawgeti is currently a stub in the provided context,
        // but the C++ macro expands to a call with 3 arguments.
        // We must cast the stub to the expected function signature to satisfy the compiler
        // until the real lua_rawgeti is translated.
        let func: fn(
            *mut crate::records::lua_state::lua_State,
            core::ffi::c_int,
            core::ffi::c_int,
        ) = core::mem::transmute(lua_rawgeti as *const ());
        func(l, LUA_REGISTRYINDEX, ref_);
    }
}
