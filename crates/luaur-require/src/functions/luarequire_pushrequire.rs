use crate::functions::lua_require::lua_require;
use crate::functions::pushrequireclosureinternal::pushrequireclosureinternal;
use crate::type_aliases::luarequire_configuration_init::luarequire_Configuration_init;
use core::ffi::c_void;
use luaur_vm::records::lua_state::lua_State;

pub fn luarequire_pushrequire(
    l: *mut lua_State,
    config_init: luarequire_Configuration_init,
    ctx: *mut c_void,
) -> i32 {
    pushrequireclosureinternal(l, config_init, ctx, Some(lua_require), c"require".as_ptr())
}
