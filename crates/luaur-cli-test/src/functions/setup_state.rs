use crate::functions::create_cli_require_context::create_cli_require_context;
use crate::functions::lua_collectgarbage::lua_collectgarbage;
use crate::functions::lua_loadstring::lua_loadstring;
use crate::functions::require_config_init::require_config_init;
use luaur_require::functions::luaopen_require::luaopen_require;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;

pub fn setup_state(l: *mut lua_State) {
    unsafe {
        luaur_vm::functions::lua_l_openlibs::lua_l_openlibs(l);

        // C++ setupState registers a few globals (loadstring/collectgarbage) onto
        // the globals table via luaL_register before opening require. (A CALLGRIND
        // build also registers "callgrind"; the default build registers only these
        // two.)
        lua_pushcclosurek(l, Some(lua_loadstring), c"loadstring".as_ptr(), 0, None);
        lua_setglobal(l, c"loadstring".as_ptr());

        lua_pushcclosurek(
            l,
            Some(lua_collectgarbage),
            c"collectgarbage".as_ptr(),
            0,
            None,
        );
        lua_setglobal(l, c"collectgarbage".as_ptr());

        let ctx = create_cli_require_context(l);
        luaopen_require(l, Some(require_config_init), ctx);

        luaur_vm::functions::lua_l_sandbox::lua_l_sandbox(l);
    }
}
