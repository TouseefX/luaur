use luaur_vm::type_aliases::lua_state::lua_State;

#[derive(Debug)]
pub struct ThreadPopper {
    pub(crate) L: *mut lua_State,
}

impl Drop for ThreadPopper {
    fn drop(&mut self) {
        unsafe {
            luaur_vm::macros::lua_pop::lua_pop(self.L, 1);
        }
    }
}
