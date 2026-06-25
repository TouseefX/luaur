use crate::functions::k_int_64_tag::K_INT_64_TAG;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::macros::lua_l_getmetatable::luaL_getmetatable;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn push_int_64(L: *mut lua_State, value: i64) {
    unsafe {
        let p = lua_newuserdatatagged(L, core::mem::size_of::<i64>(), K_INT_64_TAG);

        luaL_getmetatable(L, c"int64".as_ptr());
        lua_setmetatable(L, -2);

        *(p as *mut i64) = value;
    }
}
