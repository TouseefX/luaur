use crate::functions::convert_repl_requirer_alt_b::{convert, luarequire_ConfigStatus};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::c_void;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn get_config_status(_l: *mut lua_State, ctx: *mut c_void) -> luarequire_ConfigStatus {
    let req = &*(ctx as *const ReplRequirer);
    convert(req.vfs.get_config_status())
}
