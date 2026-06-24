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

        // The lua_touserdatatagged stub in luau-vm currently has a placeholder signature.
        // We must cast it to the real signature to call it with arguments as the C++ source does.
        type LuaToUserdataTagged = unsafe extern "C" fn(
            *mut luaur_vm::records::lua_state::lua_State,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> *mut core::ffi::c_void;

        let func: LuaToUserdataTagged = core::mem::transmute(lua_touserdatatagged as *const ());
        let result = func(
            l as *mut luaur_vm::records::lua_state::lua_State,
            idx,
            K_TYPE_USERDATA_TAG,
        );

        !result.is_null()
    }
}
