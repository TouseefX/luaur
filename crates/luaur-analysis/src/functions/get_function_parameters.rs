use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::functions::push_type_pack::push_type_pack;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::type_aliases::lua_state::lua_State;
use core::ffi::c_int;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

#[allow(non_snake_case)]
pub unsafe fn get_function_parameters(l: *mut lua_State) -> c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.parameters: expected 1 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_type_function_type_id::<TypeFunctionFunctionType>(self_ty);

    if tfft.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.parameters: expected self to be a function, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    push_type_pack(l, (*tfft).arg_types);

    1
}
