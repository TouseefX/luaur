use crate::functions::conformance_native_userdata_remapper::conformance_native_userdata_remapper;
use crate::functions::setup_userdata_helpers::setupUserdataHelpers;
use crate::functions::setup_vector_helpers::setup_vector_helpers;
use luaur_code_gen::functions::set_userdata_remapper::set_userdata_remapper;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_native_userdata_setup(l: *mut lua_State) {
    set_userdata_remapper(
        l,
        core::ptr::null_mut(),
        conformance_native_userdata_remapper,
    );

    setup_vector_helpers(l);
    setupUserdataHelpers(l);
}
