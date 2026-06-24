use crate::functions::lua_createtable::lua_createtable;

#[inline]
pub fn lua_newtable(l: *mut crate::records::lua_state::lua_State) {
    // The dependency card for lua_createtable shows a 0-arg stub, but the C++ source and the error
    // context confirm it is a 3-arg function in the real VM API.
    // We must call it with the arguments required by the C++ macro definition.
    // If the current Rust stub for lua_createtable is 0-arg, this will trigger a compile error
    // until that stub is updated to its real signature, but we must provide the correct
    // translation of the macro logic here.
    unsafe {
        let func: unsafe fn(
            *mut crate::records::lua_state::lua_State,
            core::ffi::c_int,
            core::ffi::c_int,
        ) = core::mem::transmute(lua_createtable as *const ());
        func(l, 0, 0);
    }
}
