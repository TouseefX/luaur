use crate::functions::setup_native_helpers::setup_native_helpers;
use crate::functions::setup_vector_helpers::setup_vector_helpers;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_vector_setup(l: *mut lua_State) {
    setup_vector_helpers(l);
    setup_native_helpers(l);
}
