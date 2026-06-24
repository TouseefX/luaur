use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

pub unsafe fn get_negated_value(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.inner: expected 1 argument, but got {}",
                argument_count
            ),
        );
    }

    let self_ty: TypeFunctionTypeId = get_type_user_data(l, 1);
    let tfnt = get_type_function_type_id::<TypeFunctionNegationType>(self_ty);

    if !tfnt.is_null() {
        alloc_type_user_data(l, (*(*tfnt).type_id).type_variant.clone(), false);
    } else {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.inner: cannot call inner method on non-negation type: `{}` type",
                get_tag(l, self_ty)
            ),
        );
    }

    1
}
