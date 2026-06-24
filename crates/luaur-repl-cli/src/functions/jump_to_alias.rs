use crate::functions::convert_repl_requirer::{convert, luarequire_NavigateResult};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::{c_char, c_void, CStr};
use luaur_cli_lib::functions::is_absolute_path::is_absolute_path;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn jump_to_alias(
    _l: *mut lua_State,
    ctx: *mut c_void,
    path: *const c_char,
) -> luarequire_NavigateResult {
    let req = &mut *(ctx as *mut ReplRequirer);

    let path = CStr::from_ptr(path).to_string_lossy();
    if !is_absolute_path(&path) {
        return luarequire_NavigateResult::NAVIGATE_NOT_FOUND;
    }

    convert(req.vfs.reset_to_path(&path))
}
