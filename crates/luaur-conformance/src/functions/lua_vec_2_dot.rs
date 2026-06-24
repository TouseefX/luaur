use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::records::vec_2_conformance_ir_hooks::Vec2;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_vec_2_dot(L: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
    unsafe {
        let b_ptr = lua_vec_2_get(L, 2);

        let self_val = &*self_ptr;
        let b_val = &*b_ptr;

        let result = (self_val.x as f64 * b_val.x as f64) + (self_val.y as f64 * b_val.y as f64);

        lua_pushnumber(L, result);
    }
    1
}
