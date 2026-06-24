use crate::type_aliases::lua_state::lua_State;

#[no_mangle]
pub unsafe fn lua_isyieldable(l: *mut lua_State) -> core::ffi::c_int {
    if (*l).nCcalls <= (*l).baseCcalls {
        1
    } else {
        0
    }
}
