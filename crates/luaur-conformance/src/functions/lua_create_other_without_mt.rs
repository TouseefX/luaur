use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vec_2_direct_field_access_test::Vec2;

const kTagOther: i32 = 13;

#[allow(non_snake_case)]
pub fn lua_create_other_without_mt(L: *mut lua_State) -> i32 {
    unsafe {
        lua_newuserdatatagged(
            L,
            core::mem::size_of::<Vec2>(),
            kTagOther as core::ffi::c_int,
        );
    }
    1
}
