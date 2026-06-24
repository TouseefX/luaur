use core::ffi::c_int;

use luaur_vm::functions::lua_l_checknumber::lua_l_checknumber;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::direct_field_access_k_tag_vec_2::K_TAG_VEC2;
use crate::records::vec_2_direct_field_access_test::Vec2;

pub unsafe fn direct_field_access_create_vec_2(L: *mut lua_State) -> c_int {
    let x = lua_l_checknumber(L, 1);
    let y = lua_l_checknumber(L, 2);

    let p = lua_newuserdatatagged(L, core::mem::size_of::<Vec2>(), K_TAG_VEC2) as *mut Vec2;
    (*p).x = x;
    (*p).y = y;

    1
}
