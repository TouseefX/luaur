use crate::functions::convert_repl_requirer::{convert, luarequire_NavigateResult};
use crate::records::repl_requirer::ReplRequirer;
use core::ffi::{c_char, c_void, CStr};
use luaur_vm::type_aliases::lua_state::lua_State;

pub unsafe fn to_child(
    _l: *mut lua_State,
    ctx: *mut c_void,
    name: *const c_char,
) -> luarequire_NavigateResult {
    let req = &mut *(ctx as *mut ReplRequirer);
    let name = CStr::from_ptr(name).to_string_lossy();
    convert(req.vfs.to_child(&name))
}
