use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_requireinternal::lua_requireinternal;

pub fn lua_proxyrequire(l: *mut lua_State) -> i32 {
    let requirer_chunkname = luaL_checkstring!(l, 2);
    lua_requireinternal(l, requirer_chunkname)
}
