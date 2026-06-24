use crate::functions::get_tag::get_tag;
use crate::functions::get_type_function_runtime_alt_o::get_type_function_type_id;
use crate::functions::get_type_user_data::get_type_user_data;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_pushlstring::lua_pushlstring;
use luaur_vm::functions::lua_pushnil::lua_pushnil;

pub unsafe fn get_generic_name(l: *mut lua_State) -> core::ffi::c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let self_ty = get_type_user_data(l, 1);

    let tfgt = get_type_function_type_id::<TypeFunctionGenericType>(self_ty);
    if tfgt.is_null() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!(
                "type.name: expected self to be a generic, but got {} instead",
                get_tag(l, self_ty)
            ),
        );
    }

    if (*tfgt).is_named {
        let pushlstring: unsafe extern "C" fn(
            *mut luaur_vm::records::lua_state::lua_State,
            *const core::ffi::c_char,
            usize,
        ) = core::mem::transmute(lua_pushlstring as *const ());
        pushlstring(
            vm_l,
            {
                let n = &(*tfgt).name;
                n.as_ptr() as *const core::ffi::c_char
            },
            {
                let n = &(*tfgt).name;
                n.len()
            },
        );
    } else {
        lua_pushnil(vm_l);
    }

    1
}
