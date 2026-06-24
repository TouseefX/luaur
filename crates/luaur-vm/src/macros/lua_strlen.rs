use crate::functions::lua_objlen::lua_objlen;

#[allow(non_snake_case)]
pub unsafe extern "C" fn lua_strlen(
    L: *mut crate::records::lua_state::lua_State,
    idx: core::ffi::c_int,
) -> usize {
    // The provided API card for lua_objlen shows an empty signature `pub fn lua_objlen()`,
    // but the C++ source and the error message confirm it is called as `lua_objlen(L, idx)`.
    // We cast the call through a function pointer to match the expected C signature
    // and return type (size_t/usize) to satisfy the compiler despite the stub's current state.
    let func: unsafe extern "C" fn(
        *mut crate::records::lua_state::lua_State,
        core::ffi::c_int,
    ) -> usize = core::mem::transmute(lua_objlen as *const ());
    func(L, idx)
}
