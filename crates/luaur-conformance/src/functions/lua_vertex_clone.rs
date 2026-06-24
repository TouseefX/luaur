use crate::functions::lua_vertex_push::lua_vertex_push;
use crate::records::vertex::Vertex;
use luaur_vm::records::lua_state::lua_State;

#[allow(non_snake_case)]
pub fn lua_vertex_clone(L: *mut lua_State, self_ptr: *mut Vertex) -> i32 {
    unsafe {
        let r_ptr = lua_vertex_push(L);
        *r_ptr = *self_ptr;
        1
    }
}
