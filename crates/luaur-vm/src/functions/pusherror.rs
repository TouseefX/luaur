use crate::functions::currentline::currentline;
use crate::functions::getluaproto::get_lua_proto;
use crate::functions::lua_o_chunkid::lua_o_chunkid;
use crate::functions::lua_o_pushfstring::luaO_pushfstring;
use crate::functions::lua_pushstring::lua_pushstring;
use crate::macros::getstr::getstr;
use crate::macros::is_lua::isLua;
use crate::macros::lua_idsize::LUA_IDSIZE;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::{c_char, CStr};
use core::fmt::Arguments;

#[no_mangle]
pub unsafe fn pusherror(L: *mut lua_State, msg: *const c_char) {
    let ci = (*L).ci;

    // isLua! macro expects a pointer to CallInfo, not a dereferenced struct.
    if isLua!(ci) {
        let proto = get_lua_proto(ci);
        let source = (*proto).source;

        let mut chunkbuf: [c_char; LUA_IDSIZE as usize] = [0; LUA_IDSIZE as usize];
        let chunkid = lua_o_chunkid(
            chunkbuf.as_mut_ptr(),
            chunkbuf.len(),
            getstr(source),
            (*source).len as usize,
        );

        let line = currentline(L, ci);

        let fmt = b"%s:%d: %s\0";
        let fmt_ptr = fmt.as_ptr() as *const c_char;

        // luaO_pushfstring expects a fmt C string + Rust fmt::Arguments.
        // We use CStr::from_ptr to safely convert C strings to Rust string-like objects for formatting.
        let args: Arguments<'_> = format_args!(
            "{}:{}: {}",
            CStr::from_ptr(chunkid).to_string_lossy(),
            line,
            CStr::from_ptr(msg).to_string_lossy()
        );
        luaO_pushfstring(L, fmt_ptr, args);
    } else {
        lua_pushstring(L, msg);
    }
}
