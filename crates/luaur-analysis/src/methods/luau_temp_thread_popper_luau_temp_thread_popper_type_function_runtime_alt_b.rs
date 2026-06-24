use crate::records::luau_temp_thread_popper::LuauTempThreadPopper;
use core::ffi::c_int;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::records::lua_state::lua_State as VmLuaState;

impl LuauTempThreadPopper {
    pub fn LuauTempThreadPopper(&mut self) {
        unsafe {
            lua_pop(self.L as *mut VmLuaState, 1 as c_int);
        }
    }
}
