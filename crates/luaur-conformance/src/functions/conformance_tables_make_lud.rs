use core::ffi::{c_int, c_void};

use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_l_checkunsigned::lua_l_checkunsigned;
use luaur_vm::functions::lua_topointer::lua_topointer;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::macros::lua_pushlightuserdata::lua_pushlightuserdata;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_tables_make_lud(L: *mut lua_State) -> c_int {
    if lua_type(L, 1) == lua_Type::LUA_TNUMBER as c_int {
        let v = lua_l_checkunsigned(L, 1);
        lua_pushlightuserdata(L as *mut c_void, v as usize as *mut c_void);
    } else {
        let p = lua_topointer(L, 1);
        assert!(!p.is_null());
        lua_pushlightuserdata(L as *mut c_void, p as *mut c_void);
    }

    1
}
