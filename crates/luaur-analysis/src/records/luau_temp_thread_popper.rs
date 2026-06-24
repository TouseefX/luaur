use crate::type_aliases::lua_state::lua_State;

#[derive(Debug, Clone)]
pub struct LuauTempThreadPopper {
    pub(crate) L: *mut lua_State,
}
