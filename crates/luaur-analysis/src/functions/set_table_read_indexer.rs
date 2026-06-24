use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

#[allow(non_snake_case)]
pub unsafe fn set_table_read_indexer(_L: *mut lua_State) -> core::ffi::c_int {
    let msg =
        "type.setreadindexer: luau does not yet support separate read/write types for indexers.";
    let fmt = c"%s";
    lua_l_error_l(
        _L as *mut luaur_vm::records::lua_state::lua_State,
        fmt.as_ptr(),
        core::format_args!("{}", msg),
    );
    0
}
