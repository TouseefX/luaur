use crate::functions::allocate_type_function_type::allocate_type_function_type;
use crate::records::type_function_type::TypeFunctionType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use luaur_vm::functions::lua_l_checkstack::lua_l_checkstack;
use luaur_vm::functions::lua_newuserdatatagged::lua_newuserdatatagged;
use luaur_vm::functions::lua_setmetatable::lua_setmetatable;
use luaur_vm::macros::lua_l_getmetatable::luaL_getmetatable;

const K_TYPE_USERDATA_TAG: i32 = 42;

pub fn alloc_type_user_data(
    l: *mut lua_State,
    type_variant: TypeFunctionTypeVariant,
    frozen: bool,
) {
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

        let type_id = allocate_type_function_type(l, type_variant);
        *ptr = type_id;

        let type_ptr = *ptr as *mut TypeFunctionType;
        (*type_ptr).frozen = frozen;

        luaL_getmetatable(
            l as *mut luaur_vm::records::lua_state::lua_State,
            c"type".as_ptr(),
        );
        lua_setmetatable(l as *mut luaur_vm::records::lua_state::lua_State, -2);
    }
}
