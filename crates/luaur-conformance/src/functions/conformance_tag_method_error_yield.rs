use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_tag_method_error_yield(_l: *mut lua_State) -> bool {
    true
}
