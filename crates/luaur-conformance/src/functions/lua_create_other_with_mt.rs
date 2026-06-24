use luaur_vm::functions::lua_newuserdatataggedwithmetatable::lua_newuserdatataggedwithmetatable;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vec_2_direct_field_access_test::Vec2;

const kTagOther: i32 = 13;

#[allow(non_snake_case)]
pub fn lua_create_other_with_mt(L: *mut lua_State) -> i32 {
    unsafe {
        lua_newuserdatataggedwithmetatable(
            L,
            core::mem::size_of::<Vec2>(),
            kTagOther as core::ffi::c_int,
        );
    }
    1
}
