use luaur_vm::functions::lua_getuserdatametatable::lua_getuserdatametatable;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::records::lua_state::lua_State;

use crate::records::vertex::Vertex;

pub const kTagVertex: u8 = 13;

#[allow(non_snake_case)]
pub fn lua_vertex_push(L: *mut lua_State) -> *mut Vertex {
    unsafe {
        let data = lua_newuserdatatagged(L, core::mem::size_of::<Vertex>(), kTagVertex as i32)
            as *mut Vertex;

        lua_getuserdatametatable(L, kTagVertex as i32);
        lua_setmetatable(L, -2);

        data
    }
}
