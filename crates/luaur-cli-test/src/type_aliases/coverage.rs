#[allow(non_camel_case_types)]
pub type Coverage =
    unsafe extern "C" fn(*mut luaur_vm::records::lua_state::lua_State, core::ffi::c_int);
