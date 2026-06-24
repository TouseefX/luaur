use crate::functions::lua_o_pushvfstring::luaO_pushvfstring;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::c_char;

pub fn luaO_pushfstring(
    L: *mut lua_State,
    fmt: *const c_char,
    args: core::fmt::Arguments<'_>,
) -> *const c_char {
    // In the Luau Rust port, printf-style varargs are handled by passing core::fmt::Arguments.
    // The dependency luaO_pushvfstring is a stub that expects a *mut c_void to represent the va_list.
    // We transmute the Arguments reference to match the expected pointer type of the dependency.
    unsafe {
        let lua_o_pushvfstring_fn: unsafe fn(
            *mut lua_State,
            *const c_char,
            core::fmt::Arguments<'_>,
        ) -> *const c_char = core::mem::transmute(luaO_pushvfstring as *const ());

        lua_o_pushvfstring_fn(L, fmt, args)
    }
}
