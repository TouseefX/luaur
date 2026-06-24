use luaur_vm::records::lua_state::lua_State;

use crate::functions::clear_cache_entry::clear_cache_entry;

pub fn luarequire_clearcacheentry(l: *mut lua_State) -> i32 {
    clear_cache_entry(l)
}
