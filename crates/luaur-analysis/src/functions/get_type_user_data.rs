use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use luaur_vm::functions::lua_l_typeerror_l::lua_l_typeerror_l;
use luaur_vm::functions::lua_touserdatatagged::lua_touserdatatagged;

// kTypeUserdataTag is a constant used for Luau Type Function userdata.
const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn get_type_user_data(l: *mut lua_State, idx: i32) -> TypeFunctionTypeId {
    unsafe {
        let typ = lua_touserdatatagged(
            l as *mut luaur_vm::records::lua_state::lua_State,
            idx,
            K_TYPE_USERDATA_TAG,
        ) as *mut TypeFunctionTypeId;

        if !typ.is_null() {
            return *typ;
        }

        lua_l_typeerror_l(
            l as *mut luaur_vm::records::lua_state::lua_State,
            idx,
            "type",
        );
    }
}
