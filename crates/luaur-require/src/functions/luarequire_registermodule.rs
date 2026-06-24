use luaur_vm::records::lua_state::lua_State;

use crate::functions::register_module_impl::register_module_impl;

pub fn luarequire_registermodule(l: *mut lua_State) -> i32 {
    register_module_impl(l)
}
