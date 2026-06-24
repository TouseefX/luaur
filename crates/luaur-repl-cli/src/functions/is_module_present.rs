use crate::records::repl_requirer::ReplRequirer;
use core::ffi::c_void;
use luaur_cli_lib::functions::is_file::is_file;
use luaur_code_gen::type_aliases::lua_state::lua_State;

pub unsafe fn is_module_present(_L: *mut lua_State, ctx: *mut c_void) -> bool {
    let req = &*(ctx as *const ReplRequirer);
    let path = req.vfs.get_file_path();
    is_file(&path)
}
