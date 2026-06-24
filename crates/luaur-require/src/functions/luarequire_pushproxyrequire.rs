use crate::functions::lua_proxyrequire::lua_proxyrequire;
use crate::functions::pushrequireclosureinternal::pushrequireclosureinternal;
use crate::type_aliases::luarequire_configuration_init::luarequire_Configuration_init;
use core::ffi::c_void;
use luaur_vm::records::lua_state::lua_State;

pub fn luarequire_pushproxyrequire(
    l: *mut lua_State,
    config_init: luarequire_Configuration_init,
    ctx: *mut c_void,
) -> i32 {
    pushrequireclosureinternal(
        l,
        config_init,
        ctx,
        Some(lua_proxyrequire),
        c"proxyrequire".as_ptr(),
    )
}
