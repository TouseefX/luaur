use crate::type_aliases::lua_state::lua_State;

#[allow(non_snake_case)]
#[allow(unused_variables)]
pub fn dealloc_type_user_data(L: *mut lua_State, data: *mut core::ffi::c_void) {
    // only non-owning pointers into an arena is stored
}
