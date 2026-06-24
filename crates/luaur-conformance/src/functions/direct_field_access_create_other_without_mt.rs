use core::ffi::c_int;

use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::type_aliases::lua_state::lua_State;

use crate::functions::direct_field_access_k_tag_other::K_TAG_OTHER;
use crate::records::vec_2_direct_field_access_test::Vec2;

pub unsafe fn direct_field_access_create_other_without_mt(L: *mut lua_State) -> c_int {
    lua_newuserdatatagged(L, core::mem::size_of::<Vec2>(), K_TAG_OTHER);
    1
}
