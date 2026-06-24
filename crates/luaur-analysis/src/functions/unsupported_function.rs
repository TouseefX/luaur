use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

#[allow(non_snake_case)]
pub unsafe fn unsupported_function(L: *mut lua_State) -> core::ffi::c_int {
    let msg = "this function is not supported in type functions";
    let fmt = c"%s";
    lua_l_error_l(
        L as *mut luaur_vm::records::lua_state::lua_State,
        fmt.as_ptr(),
        core::format_args!("{}", msg),
    );
    0
}
