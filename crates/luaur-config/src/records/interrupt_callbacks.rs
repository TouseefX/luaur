use luaur_vm::type_aliases::lua_state::lua_State;

#[derive(Clone)]
pub struct InterruptCallbacks {
    pub init_callback: Option<alloc::rc::Rc<dyn Fn(*mut lua_State)>>,
    pub interrupt_callback:
        Option<unsafe extern "C-unwind" fn(l: *mut lua_State, gc: core::ffi::c_int)>,
}

impl core::fmt::Debug for InterruptCallbacks {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("InterruptCallbacks")
            .field("init_callback", &self.init_callback.as_ref().map(|_| "..."))
            .field(
                "interrupt_callback",
                &self
                    .interrupt_callback
                    .map(|f| f as *const core::ffi::c_void),
            )
            .finish()
    }
}

impl Default for InterruptCallbacks {
    fn default() -> Self {
        Self {
            init_callback: None,
            interrupt_callback: None,
        }
    }
}
