use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::records::type_function_generic_type::TypeFunctionGenericType;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use core::ffi::c_int;
use core::ffi::CStr;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_l_optboolean::lua_l_optboolean;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;

pub unsafe fn create_generic(l: *mut lua_State) -> c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;
    let name_ptr = luaL_checkstring!(vm_l, 1);
    let is_pack = lua_l_optboolean(vm_l, 2, false);

    let name_cstr = CStr::from_ptr(name_ptr);
    if name_cstr.to_bytes().is_empty() {
        lua_l_error_l(
            vm_l,
            c"%s".as_ptr(),
            core::format_args!("types.generic: generic name cannot be empty"),
        );
    }

    let generic_type = TypeFunctionGenericType {
        is_named: true,
        is_pack,
        name: name_cstr.to_string_lossy().into_owned(),
    };

    alloc_type_user_data(l, TypeFunctionTypeVariant::Generic(generic_type), false);

    1
}
