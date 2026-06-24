use crate::functions::conformance_interrupt_inspection_interrupt::conformance_interrupt_inspection_interrupt;
use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_interrupt_inspection_setup(l: *mut lua_State) {
    (*lua_callbacks(l)).interrupt = Some(conformance_interrupt_inspection_interrupt);
}
