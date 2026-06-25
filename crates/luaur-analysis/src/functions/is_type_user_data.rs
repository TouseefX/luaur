use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_isuserdata::lua_isuserdata;
use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;

pub fn is_type_user_data(l: *mut lua_State, idx: i32) -> bool {
    // kTypeUserdataTag is a constant used for Luau Type Function userdata.
    const K_TYPE_USERDATA_TAG: i32 = 42;

    unsafe {
        if lua_isuserdata(l as *mut luaur_vm::records::lua_state::lua_State, idx) == 0 {
            return false;
        }

        let result = lua_touserdatatagged(
            l as *mut luaur_vm::records::lua_state::lua_State,
            idx,
            K_TYPE_USERDATA_TAG,
        );

        !result.is_null()
    }
}
