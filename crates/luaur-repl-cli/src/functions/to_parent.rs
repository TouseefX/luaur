use crate::functions::convert_repl_requirer::{convert, luarequire_NavigateResult};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::c_void;
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn to_parent(_l: *mut lua_State, ctx: *mut c_void) -> luarequire_NavigateResult {
    let req = &mut *(ctx as *mut ReplRequirer);
    convert(req.vfs.to_parent())
}
