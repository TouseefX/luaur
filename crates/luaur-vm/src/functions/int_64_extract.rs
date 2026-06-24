use crate::functions::lua_l_checkinteger_64::lua_l_checkinteger_64;
use crate::functions::lua_l_optinteger_64::lua_l_optinteger_64;
use crate::functions::lua_pushinteger_64::lua_pushinteger_64;
use crate::macros::lua_l_argcheck::luaL_argcheck;
use crate::macros::lua_l_error::luaL_error;
use crate::macros::mask_64::mask64;
use crate::type_aliases::lua_state::LuaState;

#[no_mangle]
pub unsafe fn int64_extract(l: *mut LuaState) -> core::ffi::c_int {
    let n = lua_l_checkinteger_64(l, 1);
    let f = lua_l_checkinteger_64(l, 2);
    let w = lua_l_optinteger_64(l, 3, 1);

    luaL_argcheck!(
        l,
        0 <= f as i64 && f as i64 <= 63,
        2,
        "field cannot be negative"
    );
    luaL_argcheck!(l, 0 < w as i64, 3, "width must be positive");
    if f as i64 + w as i64 > 64 {
        luaL_error!(l, "trying to access non-existent bits");
    }

    lua_pushinteger_64(l, (((n as u64) >> f as u32) & mask64(w as i32)) as i64);

    1
}
