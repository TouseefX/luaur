use luaur_vm::functions::lua_getuserdatametatable::lua_getuserdatametatable;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vec_2_conformance_ir_hooks::Vec2;

pub const kTagVec2: i32 = 12;

#[allow(non_snake_case)]
pub fn lua_vec_2_push(L: *mut lua_State) -> *mut Vec2 {
    unsafe {
        let data = lua_newuserdatatagged(
            L,
            core::mem::size_of::<Vec2>(),
            kTagVec2 as core::ffi::c_int,
        ) as *mut Vec2;

        lua_getuserdatametatable(L, kTagVec2 as core::ffi::c_int);

        lua_setmetatable(L, -2);

        data
    }
}
