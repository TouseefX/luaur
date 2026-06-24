use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_function_type::TypeFunctionFunctionType;
use crate::records::type_function_negation_type::TypeFunctionNegationType;
use crate::records::type_function_table_type::TypeFunctionTableType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

#[allow(non_snake_case)]
pub unsafe fn create_negation(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "types.negationof: expected 1 argument, but got {}",
                argument_count
            ),
        );
    }

    let arg = get_type_user_data(l, 1);

    let table_type_ptr = get_type_function_type_id::<TypeFunctionTableType>(arg);
    let function_type_ptr = get_type_function_type_id::<TypeFunctionFunctionType>(arg);

    if !table_type_ptr.is_null() || !function_type_ptr.is_null() {
        let tag = get_tag(l, arg);
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "types.negationof: cannot perform negation on `{}` type",
                tag
            ),
        );
    }

    let negation = TypeFunctionNegationType { type_id: arg };
    alloc_type_user_data(l, TypeFunctionTypeVariant::Negation(negation), false);

    1
}
