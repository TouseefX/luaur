use core::ffi::{c_char, c_void};
use luaur_code_gen::type_aliases::lua_state::lua_State;

pub unsafe extern "C" fn is_require_allowed(
    _L: *mut lua_State,
    _ctx: *mut c_void,
    requirer_chunkname: *const c_char,
) -> bool {
    if requirer_chunkname.is_null() {
        return false;
    }
    let chunkname = core::ffi::CStr::from_ptr(requirer_chunkname).to_bytes();
    chunkname == b"=stdin" || (!chunkname.is_empty() && chunkname[0] == b'@')
}
