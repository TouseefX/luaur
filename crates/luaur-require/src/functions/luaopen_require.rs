use core::ffi::c_void;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::luarequire_pushrequire::luarequire_pushrequire;
use crate::type_aliases::luarequire_configuration_init::luarequire_Configuration_init;

pub fn luaopen_require(
    l: *mut lua_State,
    config_init: luarequire_Configuration_init,
    ctx: *mut c_void,
) {
    luarequire_pushrequire(l, config_init, ctx);
    unsafe {
        lua_setglobal(l, c"require".as_ptr());
    }
}
