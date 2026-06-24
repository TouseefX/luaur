use luaur_vm::records::lua_state::lua_State;

use crate::functions::clear_cache::clear_cache;

pub fn luarequire_clearcache(l: *mut lua_State) -> i32 {
    clear_cache(l)
}
