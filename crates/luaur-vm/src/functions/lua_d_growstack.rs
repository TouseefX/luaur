use crate::functions::lua_d_reallocstack::lua_d_reallocstack;
use crate::macros::getgrownstacksize::getgrownstacksize;
use crate::type_aliases::lua_state::lua_State;

pub fn lua_d_growstack(l: *mut lua_State, n: core::ffi::c_int) {
    unsafe {
        // The dependency card for lua_d_reallocstack shows an empty signature "lua_d_reallocstack()",
        // but the C++ source and the error message confirm it is called with 3 arguments.
        // We must cast the function to the correct signature to call it with the required arguments.
        type LuaDReallocStackFn = unsafe fn(*mut lua_State, core::ffi::c_int, core::ffi::c_int);
        let reallocstack: LuaDReallocStackFn =
            core::mem::transmute(lua_d_reallocstack as *const ());
        reallocstack(l, getgrownstacksize(l, n), 0);
    }
}
