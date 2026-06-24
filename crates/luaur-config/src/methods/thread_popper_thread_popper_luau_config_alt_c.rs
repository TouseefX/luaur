use crate::records::thread_popper::ThreadPopper;

use luaur_vm::macros::lua_pop::lua_pop;

impl ThreadPopper {
    pub fn thread_popper_destructor(&mut self) {
        unsafe {
            lua_pop(self.L, 1);
        }
    }
}
