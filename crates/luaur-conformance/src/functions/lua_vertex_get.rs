use crate::records::vertex::Vertex;

use luaur_vm::functions::lua_l_typeerror_l::lua_l_typeerror_l;
use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;

const kTagVertex: i32 = 13;

#[allow(non_snake_case)]
pub fn lua_vertex_get(L: *mut luaur_vm::records::lua_state::lua_State, idx: i32) -> *mut Vertex {
    unsafe {
        let a = lua_touserdatatagged(L, idx as i32, kTagVertex) as *mut Vertex;

        if !a.is_null() {
            return a;
        }

        lua_l_typeerror_l(L, idx as i32, "vertex");
        unreachable!()
    }
}
