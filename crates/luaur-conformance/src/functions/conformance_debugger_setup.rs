use core::sync::atomic::Ordering;

use crate::functions::conformance_debugger_breakpoint::conformance_debugger_breakpoint;
use crate::functions::conformance_debugger_debug_break::conformance_debugger_debug_break;
use crate::functions::conformance_debugger_debug_interrupt::conformance_debugger_debug_interrupt;
use crate::functions::conformance_debugger_debug_step::conformance_debugger_debug_step;
use crate::records::conformance_debugger_state::CONFORMANCE_DEBUGGER_STATE;
use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::functions::lua_pushcclosurek::lua_pushcclosurek;
use luaur_vm::functions::lua_singlestep::lua_singlestep;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_debugger_setup(l: *mut lua_State) {
    let cb = lua_callbacks(l);

    lua_singlestep(
        l,
        if CONFORMANCE_DEBUGGER_STATE.singlestep.load(Ordering::SeqCst) {
            1
        } else {
            0
        },
    );

    (*cb).debugstep = Some(conformance_debugger_debug_step);
    (*cb).debugbreak = Some(conformance_debugger_debug_break);
    (*cb).debuginterrupt = Some(conformance_debugger_debug_interrupt);

    lua_pushcclosurek(
        l,
        Some(conformance_debugger_breakpoint),
        c"breakpoint".as_ptr(),
        0,
        None,
    );
    lua_setglobal(l, c"breakpoint".as_ptr());
}
