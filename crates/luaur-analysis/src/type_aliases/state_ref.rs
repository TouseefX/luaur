use crate::type_aliases::lua_state::lua_State;

#[allow(non_camel_case_types)]
pub type StateRef = (*mut lua_State, Option<unsafe extern "C" fn(*mut lua_State)>);
