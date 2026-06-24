use core::ffi::CStr;

use luaur_vm::functions::lua_getinfo::lua_getinfo;
use luaur_vm::records::lua_debug::LuaDebug;
use luaur_vm::records::lua_state::lua_State;

pub unsafe fn get_first_luau_frame_debug_info(L: *mut lua_State) -> Option<LuaDebug> {
    let mut level = 0;

    loop {
        let mut ar: LuaDebug = core::mem::zeroed();
        if lua_getinfo(L, level, c"sl".as_ptr(), &mut ar) == 0 {
            return None;
        }

        if !ar.what.is_null() && CStr::from_ptr(ar.what).to_bytes() == b"Lua" {
            return Some(ar);
        }

        level += 1;
    }
}
