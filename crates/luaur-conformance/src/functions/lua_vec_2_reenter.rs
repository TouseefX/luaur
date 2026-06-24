use crate::records::vec_2_conformance_ir_hooks::Vec2;
use luaur_vm::functions::lua_pcall::lua_pcall;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::macros::lua_getglobal::lua_getglobal;
use luaur_vm::macros::lua_isfunction::lua_isfunction;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_vec_2_reenter(L: *mut lua_State, self_ptr: *mut Vec2) -> i32 {
    unsafe {
        lua_getglobal(L, c"reenterCallback".as_ptr());
        lua_isfunction!(L, -1);
        lua_pcall(L, 0, 0, 0);

        let self_val = &*self_ptr;
        let result = (self_val.x as f64) + (self_val.y as f64);
        lua_pushnumber(L, result);
    }
    1
}
