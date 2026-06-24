use crate::functions::create_function::get_type_pack_runtime;
use crate::functions::get_mutable_type_function_runtime_alt_g::get_mutable_type_function_type_id;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

#[allow(non_snake_case)]
pub unsafe fn set_function_returns(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);

    if argument_count < 2 || argument_count > 3 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setreturns: expected 1-3 arguments, but got {}",
                argument_count
            ),
        );
    }

    let self_ty = get_type_user_data(l, 1);
    let tfft = get_mutable_type_function_type_id::<TypeFunctionFunctionType>(self_ty);
    if tfft.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setreturns: expected self to be a function, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    if luaur_common::FFlag::LuauTypeFunctionSupportsFrozen.get() && (*self_ty).frozen {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.setreturns: cannot be called to mutate a frozen type, use `types.copy` to make a copy"
            ),
        );
    }

    (*tfft).ret_types = get_type_pack_runtime(l, 2, 3);

    0
}
