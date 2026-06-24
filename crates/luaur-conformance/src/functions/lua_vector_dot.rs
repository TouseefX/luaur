use luaur_vm::functions::lua_l_checkvector::lua_l_checkvector;
use luaur_vm::functions::lua_pushnumber::lua_pushnumber;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_vector_dot(L: *mut lua_State) -> i32 {
    unsafe {
        let a = lua_l_checkvector(L, 1);
        let b = lua_l_checkvector(L, 2);

        let result =
            (*a.add(0)) * (*b.add(0)) + (*a.add(1)) * (*b.add(1)) + (*a.add(2)) * (*b.add(2));
        lua_pushnumber(L, result as f64);
    }
    1
}
