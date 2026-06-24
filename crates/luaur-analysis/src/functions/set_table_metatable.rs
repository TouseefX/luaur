use crate::functions::get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

pub unsafe fn set_table_metatable(l: *mut lua_State) -> i32 {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 2 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setmetatable: expected 2 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);

    let tftt = get_mutable_type_function_type_id::<TypeFunctionTableType>(self_ty);
    if tftt.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setmetatable: expected self to be a table, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setmetatable: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
            ),
        );
    }

    let arg = get_type_user_data(l, 2);
    if get_type_function_type_id::<TypeFunctionTableType>(arg).is_null() {
        let tag_ty = if luaur_common::FFlag::LuauTypeFunctionRobustness.get() {
            arg
        } else {
            self_ty
        };
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setmetatable: expected the argument to be a table, but got {} instead",
                get_tag(l, tag_ty)
            ),
        );
    }

    (*tftt).metatable = Some(arg);

    0
}
