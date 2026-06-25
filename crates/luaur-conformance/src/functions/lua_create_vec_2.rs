use luaur_vm::functions::lua_l_checknumber::lua_l_checknumber;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vec_2_conformance_ir_hooks::Vec2;

pub const kTagVec2: i32 = 12;

#[allow(non_snake_case)]
pub fn lua_create_vec_2(L: *mut lua_State) -> i32 {
    unsafe {
        let x = lua_l_checknumber(L, 1);
        let y = lua_l_checknumber(L, 2);

        let p = lua_newuserdatatagged(
            L,
            core::mem::size_of::<Vec2>(),
            kTagVec2 as core::ffi::c_int,
        ) as *mut Vec2;

        (*p).x = x as f32;
        (*p).y = y as f32;
    }
    1
}
