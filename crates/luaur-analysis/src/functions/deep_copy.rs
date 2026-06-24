use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::functions::deep_clone::deep_clone;
use crate::functions::get_type_function_runtime::get_type_function_runtime;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;

#[allow(non_snake_case)]
pub unsafe fn deep_copy(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let argument_count = lua_gettop(vm_l);
    if argument_count != 1 {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "types.copy: expected 1 arguments, but got {}",
                argument_count
            ),
        );
    }

    let arg = get_type_user_data(l, 1);
    let runtime = get_type_function_runtime(l);
    let copy = deep_clone(runtime, arg);

    if luaur_common::FFlag::LuauTypeFunctionRobustness.get() && copy.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!("types.copy: complexity limit reached during type copy"),
        );
    }

    alloc_type_user_data(l, (*copy).type_variant.clone(), false);
    1
}
