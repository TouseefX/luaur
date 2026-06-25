use luaur_vm::functions::lua_l_checkvector::lua_l_checkvector;
use luaur_vm::macros::lua_vector_size::LUA_VECTOR_SIZE;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_vector_cross(L: *mut lua_State) -> i32 {
    unsafe {
        let a = lua_l_checkvector(L, 1);
        let b = lua_l_checkvector(L, 2);

        let x = (*a.add(1)) * (*b.add(2)) - (*a.add(2)) * (*b.add(1));
        let y = (*a.add(2)) * (*b.add(0)) - (*a.add(0)) * (*b.add(2));
        let z = (*a.add(0)) * (*b.add(1)) - (*a.add(1)) * (*b.add(0));

        if LUA_VECTOR_SIZE == 4 {
            lua_pushvector_4(L, x, y, z, 0.0);
        } else {
            lua_pushvector_3(L, x, y, z);
        }

        1
    }
}

#[allow(non_snake_case)]
unsafe fn lua_pushvector_4(L: *mut lua_State, x: f32, y: f32, z: f32, w: f32) {
    luaur_vm::functions::lua_pushvector_lapi::lua_pushvector_lua_state_f32_f32_f32_f32(
        L, x, y, z, w,
    );
}

#[allow(non_snake_case)]
unsafe fn lua_pushvector_3(L: *mut lua_State, x: f32, y: f32, z: f32) {
    luaur_vm::functions::lua_pushvector_lapi_alt_b::lua_pushvector_lua_state_f32_f32_f32(
        L, x, y, z,
    );
}
