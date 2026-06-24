use core::sync::atomic::Ordering;

use crate::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_debugger_debug_step(_l: *mut lua_State, _ar: *mut LuaDebug) {
    CONFORMANCE_DEBUGGER_STATE
        .stephits
        .fetch_add(1, Ordering::SeqCst);
}
