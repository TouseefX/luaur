use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::macros::lua_l_getmetatable::luaL_getmetatable;

// kTypeUserdataTag is a constant used for Luau Type Function userdata.
const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn push_type(l: *mut lua_State, r#type: TypeFunctionTypeId) {
    unsafe {
        lua_l_checkstack(
            l as *mut luaur_vm::records::lua_state::lua_State,
            2,
            "allocating type",
        );

        let ptr = lua_newuserdatatagged(
            l as *mut luaur_vm::records::lua_state::lua_State,
            core::mem::size_of::<TypeFunctionTypeId>(),
            K_TYPE_USERDATA_TAG,
        ) as *mut TypeFunctionTypeId;

        *ptr = r#type;

        // set the new userdata's metatable to type metatable
        luaL_getmetatable(
            l as *mut luaur_vm::records::lua_state::lua_State,
            c"type".as_ptr(),
        );
        lua_setmetatable(l as *mut luaur_vm::records::lua_state::lua_State, -2);
    }
}
