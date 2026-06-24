use core::ffi::c_int;

use luaur_vm::functions::lua_breakpoint::lua_breakpoint;
use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::functions::lua_l_checkinteger::lua_l_checkinteger;
use luaur_vm::functions::lua_l_optboolean::lua_l_optboolean;
use luaur_vm::functions::lua_stackdepth::lua_stackdepth;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn conformance_debugger_breakpoint(l: *mut lua_State) -> c_int {
    let line = lua_l_checkinteger(l, 1);
    let enabled = lua_l_optboolean(l, 2, true);

    let mut ar: LuaDebug = core::mem::zeroed();
    lua_getinfo(l, lua_stackdepth(l) - 1, c"f".as_ptr(), &mut ar);

    lua_breakpoint(l, -1, line, if enabled { 1 } else { 0 });
    0
}
