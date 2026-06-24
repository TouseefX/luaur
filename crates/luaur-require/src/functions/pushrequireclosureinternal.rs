use crate::functions::lua_requirecont::lua_requirecont;
use crate::functions::validate_config::validate_config;
use crate::records::luarequire_configuration::luarequire_Configuration;
use crate::type_aliases::luarequire_configuration_init::luarequire_Configuration_init;
use core::ffi::{c_char, c_void};
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_newuserdata::lua_newuserdata;
use luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata;
use luaur_vm::records::lua_state::lua_State;
use luaur_vm::type_aliases::lua_c_function::lua_CFunction;

pub fn pushrequireclosureinternal(
    l: *mut lua_State,
    config_init: luarequire_Configuration_init,
    ctx: *mut c_void,
    requirelikefunc: lua_CFunction,
    debugname: *const c_char,
) -> i32 {
    unsafe {
        let ud = lua_newuserdata(l, core::mem::size_of::<luarequire_Configuration>());
        if ud.is_null() {
            luaL_error!(l, "failed to allocate memory for require configuration");
            return 0;
        }

        let config = ud as *mut luarequire_Configuration;
        core::ptr::write(config, core::mem::zeroed());

        let Some(config_init) = config_init else {
            luaL_error!(
                l,
                "require configuration is missing required initializer function"
            );
            return 0;
        };

        config_init(config);
        validate_config(l, &*config);

        lua_pushlightuserdata(l as *mut c_void, ctx);
        lua_pushcclosurek(l, requirelikefunc, debugname, 2, Some(lua_requirecont));
        1
    }
}
