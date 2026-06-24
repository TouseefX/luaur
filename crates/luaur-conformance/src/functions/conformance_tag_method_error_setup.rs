use crate::functions::conformance_tag_method_error_debug_protected_error::conformance_tag_method_error_debug_protected_error;
use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_tag_method_error_setup(l: *mut lua_State) {
    (*lua_callbacks(l)).debugprotectederror =
        Some(conformance_tag_method_error_debug_protected_error);
}
