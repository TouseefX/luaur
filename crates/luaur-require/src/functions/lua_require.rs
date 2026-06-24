use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

use crate::functions::lua_requireinternal::lua_requireinternal;

pub fn lua_require(l: *mut lua_State) -> i32 {
    unsafe {
        let mut ar: LuaDebug = core::mem::zeroed();
        let mut level = 1;

        loop {
            if lua_getinfo(l, level, c"s".as_ptr(), &mut ar) == 0 {
                luaL_error!(l, "require is not supported in this context");
                return 0;
            }
            level += 1;

            if ar.what.is_null() || *ar.what != b'C' as core::ffi::c_char {
                break;
            }
        }

        lua_requireinternal(l, ar.source)
    }
}
