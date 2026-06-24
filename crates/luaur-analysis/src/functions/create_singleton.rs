use crate::enums::type_type_function_runtime::Type;
use crate::functions::alloc_type_user_data::alloc_type_user_data;
use crate::records::type_function_boolean_singleton::TypeFunctionBooleanSingleton;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;
use crate::records::type_function_singleton_type::TypeFunctionSingletonType;
use crate::records::type_function_string_singleton::TypeFunctionStringSingleton;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::type_function_singleton_variant::TypeFunctionSingletonVariant;
use crate::type_aliases::type_function_type_variant::TypeFunctionTypeVariant;
use core::ffi::c_int;
use core::ffi::CStr;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_l_checkboolean::lua_l_checkboolean;
use luaur_vm::functions::lua_l_error_l::lua_l_error_l;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::functions::lua_typename::lua_typename;
use luaur_vm::macros::lua_isboolean::lua_isboolean;
use luaur_vm::macros::lua_isnil::lua_isnil;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;

pub unsafe fn create_singleton(l: *mut lua_State) -> c_int {
    let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

    if lua_isboolean!(vm_l, 1) {
        let value = lua_l_checkboolean(vm_l, 1) != 0;
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
                variant: TypeFunctionSingletonVariant::V0(TypeFunctionBooleanSingleton { value }),
            }),
            false,
        );

        return 1;
    }

    if lua_type(vm_l, 1) == lua_Type::LUA_TSTRING as i32 {
        let value = luaL_checkstring!(vm_l, 1);
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Singleton(TypeFunctionSingletonType {
                variant: TypeFunctionSingletonVariant::V1(TypeFunctionStringSingleton {
                    value: CStr::from_ptr(value).to_string_lossy().into_owned(),
                }),
            }),
            false,
        );

        return 1;
    }

    if lua_isnil!(vm_l, 1) {
        alloc_type_user_data(
            l,
            TypeFunctionTypeVariant::Primitive(TypeFunctionPrimitiveType::new(Type::NilType)),
            false,
        );

        return 1;
    }

    let type_name = CStr::from_ptr(lua_typename(vm_l, 1)).to_string_lossy();
    lua_l_error_l(
        vm_l,
        c"%s".as_ptr(),
        core::format_args!(
            "types.singleton: can't create singleton from `{}` type",
            type_name
        ),
    );
    0
}
