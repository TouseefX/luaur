use core::sync::atomic::Ordering;

use crate::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_debugger_debug_interrupt(
    _l: *mut lua_State,
    ar: *mut LuaDebug,
) {
    assert!(CONFORMANCE_DEBUGGER_STATE
        .interruptedthread
        .load(Ordering::SeqCst)
        .is_null());
    assert!(!(*ar).userdata.is_null());

    CONFORMANCE_DEBUGGER_STATE
        .interruptedthread
        .store((*ar).userdata as *mut lua_State, Ordering::SeqCst);
}
