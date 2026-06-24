use crate::type_aliases::lua_state::lua_State;

#[allow(non_snake_case)]
pub(crate) fn dummyStateClose(_: *mut lua_State) {}
