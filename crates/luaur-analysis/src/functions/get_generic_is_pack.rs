use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushboolean::lua_pushboolean;

pub unsafe fn get_generic_is_pack(l: *mut lua_State) -> i32 {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let self_ty = get_type_user_data(l, 1);
    let tfgt = get_type_function_type_id::<TypeFunctionGenericType>(self_ty);

    if tfgt.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.ispack: expected self to be a generic, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    lua_pushboolean(vm_l, (*tfgt).is_pack as core::ffi::c_int);
    1
}
