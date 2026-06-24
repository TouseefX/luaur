use core::ffi::c_int;

use luaur_vm::functions::lua_break::lua_break;
use luaur_vm::functions::lua_isyieldable::lua_isyieldable;
use luaur_vm::records::lua_state::lua_State;

static mut SKIP_BREAK: bool = false;

pub unsafe extern "C-unwind" fn conformance_interrupt_inspection_interrupt(
    l: *mut lua_State,
    gc: c_int,
) {
    if gc >= 0 {
        return;
    }

    if lua_isyieldable(l) == 0 {
        return;
    }

    if !SKIP_BREAK {
        lua_break(l);
    }

    SKIP_BREAK = !SKIP_BREAK;
}
