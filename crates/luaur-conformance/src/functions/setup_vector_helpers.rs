use luaur_vm::functions::lua_l_newmetatable::lua_l_newmetatable;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::functions::lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32;
use luaur_vm::functions::lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::functions::lua_setreadonly::lua_setreadonly;
use luaur_vm::functions::lua_settable::lua_settable;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_vector_size::LUA_VECTOR_SIZE;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vector_index::lua_vector_index;
use crate::functions::lua_vector_namecall::lua_vector_namecall;

pub unsafe fn setup_vector_helpers(L: *mut lua_State) {
    if LUA_VECTOR_SIZE == 4 {
        lua_pushvector_lua_state_f32_f32_f32_f32(L, 0.0, 0.0, 0.0, 0.0);
    } else {
        lua_pushvector_lua_state_f32_f32_f32(L, 0.0, 0.0, 0.0);
    }

    lua_l_newmetatable(L, c"vector".as_ptr());

    lua_pushstring(L, c"__index".as_ptr());
    lua_pushcclosurek(L, Some(lua_vector_index), core::ptr::null(), 0, None);
    lua_settable(L, -3);

    lua_pushstring(L, c"__namecall".as_ptr());
    lua_pushcclosurek(L, Some(lua_vector_namecall), core::ptr::null(), 0, None);
    lua_settable(L, -3);

    lua_setreadonly(L, -1, 1);
    lua_setmetatable(L, -2);
    lua_pop(L, 1);
}
