use luaur_vm::functions::lua_l_checkvector::lua_l_checkvector;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_vec_2_get::lua_vec_2_get;
use crate::functions::lua_vertex_push::lua_vertex_push;
use crate::records::vertex::Vertex;

#[allow(non_snake_case)]
pub fn lua_vertex(L: *mut lua_State) -> i32 {
    unsafe {
        let pos = lua_l_checkvector(L, 1);
        let normal = lua_l_checkvector(L, 2);
        let uv = lua_vec_2_get(L, 3);

        let data = lua_vertex_push(L);

        (*data).pos[0] = *pos;
        (*data).pos[1] = *pos.add(1);
        (*data).pos[2] = *pos.add(2);

        (*data).normal[0] = *normal;
        (*data).normal[1] = *normal.add(1);
        (*data).normal[2] = *normal.add(2);

        (*data).uv[0] = (*uv).x;
        (*data).uv[1] = (*uv).y;

        1
    }
}
