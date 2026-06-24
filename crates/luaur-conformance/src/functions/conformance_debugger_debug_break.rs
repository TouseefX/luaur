use core::sync::atomic::Ordering;

use crate::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
use luaur_vm::functions::lua_break::lua_break;
use luaur_vm::functions::lua_debugtrace::lua_debugtrace;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_debugger_debug_break(l: *mut lua_State, _ar: *mut LuaDebug) {
    let breakhits = CONFORMANCE_DEBUGGER_STATE
        .breakhits
        .fetch_add(1, Ordering::SeqCst)
        + 1;

    lua_debugtrace(l);

    if breakhits % 2 == 1 {
        lua_break(l);
    }
}
