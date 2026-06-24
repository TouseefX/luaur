use crate::functions::lua_isyieldable::lua_isyieldable;
use crate::functions::lua_pushboolean::lua_pushboolean;
use crate::type_aliases::lua_state::lua_State;

#[no_mangle]
pub unsafe fn coyieldable(l: *mut lua_State) -> core::ffi::c_int {
    lua_pushboolean(l, lua_isyieldable(l));
    1
}
