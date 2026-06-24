use crate::records::luau_temp_thread_popper::LuauTempThreadPopper;
use crate::type_aliases::lua_state::lua_State;

impl LuauTempThreadPopper {
    pub fn new(l: *mut lua_State) -> Self {
        Self { L: l }
    }
}

#[allow(non_snake_case)]
impl LuauTempThreadPopper {
    pub fn luau_temp_thread_popper_constructor(l: *mut lua_State) -> Self {
        Self::new(l)
    }
}
