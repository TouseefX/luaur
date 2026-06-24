use crate::functions::get_type_user_data::get_type_user_data;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use luaur_vm::macros::lua_isnoneornil::lua_isnoneornil;

pub fn optional_type_user_data(l: *mut lua_State, idx: i32) -> Option<TypeFunctionTypeId> {
    unsafe {
        if lua_isnoneornil!(l as *mut luaur_vm::records::lua_state::lua_State, idx) {
            None
        } else {
            Some(get_type_user_data(l, idx))
        }
    }
}
