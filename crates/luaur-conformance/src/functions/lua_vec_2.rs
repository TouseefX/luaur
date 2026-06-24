use luaur_vm::functions::lua_l_checknumber::lua_l_checknumber;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_push::lua_vec_2_push;
use crate::records::vec_2_conformance_ir_hooks::Vec2;

#[allow(non_snake_case)]
pub fn lua_vec_2(L: *mut lua_State) -> i32 {
    unsafe {
        let x = lua_l_checknumber(L, 1);
        let y = lua_l_checknumber(L, 2);

        let data = lua_vec_2_push(L);

        (*data).x = x as f32;
        (*data).y = y as f32;
    }
    1
}
